mod github_repo;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use tauri::Emitter;
use tauri::Manager;

// use tauri::WebviewUrl;
// use tauri::WebviewWindowBuilder;
// use std::collections::HashMap;
//
use std::collections::HashSet;
use std::fs::File;
use std::panic;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use bmm_lib::balamod::find_balatros;
use bmm_lib::cache;
use bmm_lib::cache::Mod;
use bmm_lib::database::Database;
use bmm_lib::database::InstalledMod;
use bmm_lib::discord_rpc::DiscordRpcManager;
use bmm_lib::errors::AppError;
use bmm_lib::finder::is_balatro_running;
use bmm_lib::finder::is_steam_running;
use bmm_lib::lovely;
use bmm_lib::smods_installer::{ModInstaller, ModType};

fn map_error<T>(result: Result<T, AppError>) -> Result<T, String> {
    result.map_err(|e| e.to_string())
}

// Create a state structure to hold the database
struct AppState {
    db: Mutex<Database>,
    discord_rpc: Mutex<DiscordRpcManager>,
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModMeta {
    #[serde(rename = "requires-steamodded")]
    pub requires_steamodded: bool,
    #[serde(rename = "requires-talisman")]
    pub requires_talisman: bool,
    pub categories: Vec<String>,
    pub author: String,
    pub repo: String,
    pub title: String,
    #[serde(rename = "downloadURL")]
    pub download_url: Option<String>,
}

#[tauri::command]
async fn check_steam_running() -> bool {
    is_steam_running()
}

#[tauri::command]
async fn check_balatro_running() -> bool {
    is_balatro_running()
}

#[tauri::command]
async fn save_versions_cache(mod_type: String, versions: Vec<String>) -> Result<(), String> {
    map_error(cache::save_versions_cache(&mod_type, &versions))
}

#[tauri::command]
async fn get_repo_path() -> Result<String, String> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| AppError::DirNotFound(PathBuf::from("config directory")).to_string())?;
    let repo_path = config_dir.join("Balatro").join("mod_index");
    Ok(repo_path.to_string_lossy().into_owned())
}

#[tauri::command]
async fn clone_repo(url: &str, path: &str) -> Result<(), String> {
    github_repo::clone_repository(url, path).await
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModCacheInfo {
    pub path: String,
    pub last_commit: i64,
}

#[allow(non_snake_case)]
#[tauri::command]
async fn get_mod_thumbnail(modPath: String) -> Result<Option<String>, String> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| AppError::DirNotFound(PathBuf::from("config directory")).to_string())?;

    let full_path = config_dir
        .join("Balatro")
        .join("mod_index")
        .join("mods")
        .join(modPath)
        .join("thumbnail.jpg");

    // Read the image file
    let image_data = match std::fs::read(&full_path) {
        Ok(data) => data,
        Err(_) => {
            return Ok(None);
        }
    };

    // Convert to base64
    let base64 = STANDARD.encode(image_data);
    Ok(Some(format!("data:image/jpeg;base64,{}", base64)))
}

// #[allow(non_snake_case)]
// #[tauri::command]
// async fn get_mod_timestamps(repoPath: String) -> Result<HashMap<String, i64>, String> {
//     github_repo::get_mod_timestamps(&repoPath).await
// }

#[tauri::command]
async fn pull_repo(path: &str) -> Result<(), String> {
    // Check if directory exists
    let path_buf = PathBuf::from(path);
    if !path_buf.exists() {
        return Err(format!("Directory '{}' does not exist", path));
    }

    // Check if it's a repository
    if !github_repo::is_repository_directory(path) {
        // Auto-clone if it doesn't look like a repository
        let repo_url = "https://github.com/skyline69/balatro-mod-index"; // Default repository URL
        return github_repo::clone_repository(repo_url, path).await;
    }

    // Proceed with pull if it's a valid repository
    github_repo::pull_repository(path).await
}

#[tauri::command]
async fn list_directories(path: &str) -> Result<Vec<String>, String> {
    let dir = PathBuf::from(path);
    let entries = std::fs::read_dir(dir).map_err(|e| {
        AppError::FileRead {
            path: PathBuf::from(path),
            source: e.to_string(),
        }
        .to_string()
    })?;

    let mut dirs = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| {
            AppError::FileRead {
                path: PathBuf::from(path),
                source: e.to_string(),
            }
            .to_string()
        })?;

        if let Ok(file_type) = entry.file_type() {
            if file_type.is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    dirs.push(name.to_string());
                }
            }
        }
    }
    Ok(dirs)
}

#[tauri::command]
async fn read_json_file(path: &str) -> Result<ModMeta, String> {
    let path = PathBuf::from(path);
    let file = File::open(&path).map_err(|e| {
        AppError::FileRead {
            path: path.clone(),
            source: e.to_string(),
        }
        .to_string()
    })?;

    serde_json::from_reader(file).map_err(|e| {
        AppError::JsonParse {
            path,
            source: e.to_string(),
        }
        .to_string()
    })
}

#[tauri::command]
async fn read_text_file(path: &str) -> Result<String, String> {
    let path = PathBuf::from(path);
    std::fs::read_to_string(&path).map_err(|e| {
        AppError::FileRead {
            path,
            source: e.to_string(),
        }
        .to_string()
    })
}

#[tauri::command]
async fn get_last_fetched(state: tauri::State<'_, AppState>) -> Result<u64, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_last_fetched().map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_last_fetched(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.set_last_fetched(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn load_versions_cache(mod_type: String) -> Result<Option<(Vec<String>, u64)>, String> {
    cache::load_versions_cache(&mod_type)
        .map(|res| {
            res.map(|versions| {
                (
                    versions,
                    match SystemTime::now().duration_since(UNIX_EPOCH) {
                        Ok(dur) => dur,
                        Err(e) => {
                            log::error!("Failed to get current time: {}", e);
                            std::process::exit(1);
                        }
                    }
                    .as_secs(),
                )
            })
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn save_mods_cache(mods: Vec<Mod>) -> Result<(), String> {
    map_error(cache::save_cache(&mods))
}

#[tauri::command]
async fn clear_cache() -> Result<(), String> {
    map_error(cache::clear_cache())
}

#[tauri::command]
async fn load_mods_cache() -> Result<Option<(Vec<Mod>, u64)>, String> {
    map_error(cache::load_cache())
}

#[tauri::command]
async fn get_lovely_console_status(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let db = state
        .db
        .lock()
        .map_err(|_| AppError::LockPoisoned("Database lock poisoned".to_string()))?;
    map_error(db.is_lovely_console_enabled())
}

#[tauri::command]
async fn set_lovely_console_status(
    state: tauri::State<'_, AppState>,
    enabled: bool,
) -> Result<(), String> {
    let db = state
        .db
        .lock()
        .map_err(|_| AppError::LockPoisoned("Database lock poisoned".to_string()))?;
    map_error(db.set_lovely_console_status(enabled))
}

#[tauri::command]
async fn check_untracked_mods(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let mod_dir = dirs::config_dir()
        .ok_or_else(|| AppError::DirNotFound(PathBuf::from("config directory")))?
        .join("Balatro")
        .join("Mods");

    let db = state
        .db
        .lock()
        .map_err(|_| AppError::LockPoisoned("Database lock poisoned".to_string()))?;
    let installed_mods = db.get_installed_mods()?;

    let entries = match std::fs::read_dir(&mod_dir) {
        Ok(entries) => entries,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(e) => {
            return Err(AppError::FileRead {
                path: mod_dir,
                source: e.to_string(),
            }
            .to_string())
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) if !name.contains("lovely") => name,
            _ => continue,
        };

        if !installed_mods.iter().any(|m| m.path.contains(name)) {
            return Ok(true);
        }
    }

    Ok(false)
}

#[tauri::command]
async fn refresh_mods_folder(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mod_dir = dirs::config_dir()
        .ok_or_else(|| AppError::DirNotFound(PathBuf::from("config directory")))?
        .join("Balatro")
        .join("Mods");

    let db = state
        .db
        .lock()
        .map_err(|_| AppError::LockPoisoned("Database lock poisoned".to_string()))?;
    let installed_mods = db.get_installed_mods()?;

    let entries = std::fs::read_dir(&mod_dir).map_err(|e| AppError::FileRead {
        path: mod_dir.clone(),
        source: e.to_string(),
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| AppError::FileRead {
            path: mod_dir.clone(),
            source: e.to_string(),
        })?;
        let path = entry.path();
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| AppError::InvalidState("Invalid filename".to_string()))?;

        if name.contains(".lovely") || name.contains("lovely") {
            continue;
        }

        let ft = entry.file_type().map_err(|e| AppError::FileRead {
            path: path.clone(),
            source: e.to_string(),
        })?;

        match (ft.is_dir(), ft.is_file()) {
            (true, _) => {
                if !installed_mods.iter().any(|m| m.path.contains(name)) {
                    std::fs::remove_dir_all(&path).map_err(|e| AppError::FileWrite {
                        path: path.clone(),
                        source: e.to_string(),
                    })?;
                }
            }
            (_, true) => {
                if !installed_mods.iter().any(|m| m.path.contains(name)) {
                    std::fs::remove_file(&path).map_err(|e| AppError::FileWrite {
                        path: path.clone(),
                        source: e.to_string(),
                    })?;
                }
            }
            _ => continue,
        }
    }
    Ok(())
}

#[tauri::command]
async fn get_discord_rpc_status(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let db = state
        .db
        .lock()
        .map_err(|_| AppError::LockPoisoned("Database lock poisoned".to_string()))?;
    map_error(db.is_discord_rpc_enabled())
}

#[tauri::command]
async fn set_discord_rpc_status(
    state: tauri::State<'_, AppState>,
    enabled: bool,
) -> Result<(), String> {
    // Step 1: Update database setting
    let db = state
        .db
        .lock()
        .map_err(|_| AppError::LockPoisoned("Database lock poisoned".to_string()))?;

    map_error(db.set_discord_rpc_enabled(enabled))?;

    // Step 2: Update Discord RPC manager
    let discord_rpc = state
        .discord_rpc
        .lock()
        .map_err(|_| AppError::LockPoisoned("Discord RPC lock poisoned".to_string()))?;

    discord_rpc.set_enabled(enabled);

    Ok(())
}

#[tauri::command]
async fn launch_balatro(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let (path_str, lovely_console_enabled) = {
        let db = state
            .db
            .lock()
            .map_err(|_| AppError::LockPoisoned("Database lock poisoned".to_string()))?;

        (
            db.get_installation_path()?
                .ok_or_else(|| AppError::InvalidState("No installation path set".to_string()))?,
            db.is_lovely_console_enabled()?,
        )
    };

    let path = PathBuf::from(path_str);

    #[cfg(target_os = "macos")]
    {
        let lovely_path = map_error(lovely::ensure_lovely_exists().await)?;
        let balatro_executable = path.join("Balatro.app/Contents/MacOS/love");

        if lovely_console_enabled {
            // If the console is disabled, add the flag
            let disable_arg = if !lovely_console_enabled {
                " --disable-console"
            } else {
                ""
            };
            // Instead of using double quotes which cause conflicts in AppleScript,
            // wrap the file paths in single quotes.
            let command_line = format!(
                "cd '{}' && DYLD_INSERT_LIBRARIES='{}' '{}'{}",
                path.display(),
                lovely_path.display(),
                balatro_executable.display(),
                disable_arg
            );

            // Construct the AppleScript command to run the command_line in Terminal.
            let applescript = format!(
                "tell application \"Terminal\" to do script \"{}\"",
                command_line
            );

            Command::new("osascript")
                .arg("-e")
                .arg(applescript)
                .spawn()
                .map_err(|e| AppError::ProcessExecution(e.to_string()))?;
        } else {
            let mut command = Command::new(path.join("Balatro.app/Contents/MacOS/love"));
            command
                .env("DYLD_INSERT_LIBRARIES", lovely_path)
                .current_dir(&path);
            command
                .spawn()
                .map_err(|e| AppError::ProcessExecution(e.to_string()))?;
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Paths for the executable
        let exe_path = path.join("Balatro.exe");
        let dll_path = path.join("version.dll");

        // If version.dll doesn't exist, download it
        if !dll_path.exists() {
            lovely::ensure_version_dll_exists(&path).await?;
        }

        // Launch the game
        if lovely_console_enabled {
            Command::new(&exe_path)
                .current_dir(&path)
                .spawn()
                .map_err(|e| format!("Failed to launch Balatro.exe: {}", e))?;
        } else {
            Command::new(&exe_path)
                .current_dir(&path)
                .arg("--disable-console")
                .spawn()
                .map_err(|e| format!("Failed to launch Balatro.exe: {}", e))?;
        }

        log::debug!("Launched Balatro from {}", exe_path.display());
    }

    Ok(())
}

#[tauri::command]
async fn check_mod_installation(mod_type: String) -> Result<bool, String> {
    let db = map_error(Database::new())?;
    let installed_mods = map_error(db.get_installed_mods())?;

    Ok(match mod_type.as_str() {
        "Steamodded" => installed_mods.iter().any(|m| m.name == "Steamodded"),
        "Talisman" => installed_mods.iter().any(|m| m.name == "Talisman"),
        _ => return Err(AppError::InvalidState("Invalid mod type".to_string()).to_string()),
    })
}

#[tauri::command]
async fn check_existing_installation(
    state: tauri::State<'_, AppState>,
) -> Result<Option<String>, String> {
    let db = state
        .db
        .lock()
        .map_err(|_| AppError::LockPoisoned("Database lock poisoned".to_string()))?;
    if let Some(path) = db.get_installation_path()? {
        let path_buf = PathBuf::from(&path);
        if bmm_lib::balamod::Balatro::from_custom_path(path_buf).is_some() {
            Ok(Some(path))
        } else {
            map_error(db.remove_installation_path())?;
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

#[tauri::command]
async fn install_mod(url: String) -> Result<PathBuf, String> {
    map_error(bmm_lib::installer::install_mod(url).await)
}

#[tauri::command]
async fn get_installed_mods_from_db(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<InstalledMod>, String> {
    let db = state
        .db
        .lock()
        .map_err(|_| AppError::LockPoisoned("Database lock poisoned".to_string()))?;
    map_error(db.get_installed_mods())
}

#[tauri::command]
async fn add_installed_mod(
    state: tauri::State<'_, AppState>,
    name: String,
    path: String,
    dependencies: Vec<String>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    map_error(db.add_installed_mod(&name, &path, &dependencies))
}

#[tauri::command]
async fn force_remove_mod(
    state: tauri::State<'_, AppState>,
    name: String,
    path: String,
) -> Result<(), String> {
    map_error(bmm_lib::installer::uninstall_mod(PathBuf::from(path)))?;
    let db = state.db.lock().map_err(|e| e.to_string())?;
    map_error(db.remove_installed_mod(&name))
}

#[tauri::command]
async fn reindex_mods(state: tauri::State<'_, AppState>) -> Result<(usize, usize), String> {
    let mod_dir = dirs::config_dir()
        .ok_or_else(|| AppError::DirNotFound(PathBuf::from("config directory")))?
        .join("Balatro")
        .join("Mods");

    let db = state
        .db
        .lock()
        .map_err(|e| AppError::LockPoisoned(format!("Database lock poisoned: {}", e)))?;

    // Phase 1: Filesystem cleanup
    let mut files_removed = 0;
    let installed_mods = db.get_installed_mods().map_err(|e| e.to_string())?;

    match std::fs::read_dir(&mod_dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = match path.file_name().and_then(|n| n.to_str()) {
                    Some(n) if !n.contains("lovely") => n,
                    _ => continue,
                };

                if !installed_mods.iter().any(|m| m.path.contains(name)) {
                    match entry.file_type() {
                        Ok(ft) => {
                            if ft.is_dir() {
                                std::fs::remove_dir_all(&path).ok();
                            } else if ft.is_file() {
                                std::fs::remove_file(&path).ok();
                            }
                            files_removed += 1;
                        }
                        Err(_) => continue,
                    }
                }
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e.to_string()),
    }

    // Phase 2: Database cleanup
    let installed_mods = db.get_installed_mods().map_err(|e| e.to_string())?;
    let mut to_remove = Vec::new();

    for (index, installed_mod) in installed_mods.iter().enumerate() {
        if !PathBuf::from(&installed_mod.path).exists() {
            to_remove.push(index);
        }
    }

    let cleaned_entries = to_remove.len();
    for &index in to_remove.iter().rev() {
        db.remove_installed_mod(&installed_mods[index].name)
            .map_err(|e| e.to_string())?;
    }

    Ok((files_removed, cleaned_entries))
}

#[tauri::command]
async fn get_dependents(mod_name: String) -> Result<Vec<String>, String> {
    let db = Database::new().map_err(|e| e.to_string())?;
    let all_dependents = db.get_dependents(&mod_name).map_err(|e| e.to_string())?;

    // Filter out self-references
    let filtered_dependents: Vec<String> = all_dependents
        .into_iter()
        .filter(|dep| dep != &mod_name)
        .collect();

    Ok(filtered_dependents)
}

#[tauri::command]
async fn cascade_uninstall(
    state: tauri::State<'_, AppState>,
    root_mod: String,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut to_uninstall = vec![root_mod.clone()];
    let mut processed = HashSet::new();

    while let Some(current) = to_uninstall.pop() {
        if processed.contains(&current) {
            continue;
        }
        processed.insert(current.clone());

        // Get mod details
        let mod_details = map_error(db.get_mod_details(&current))?;

        // Add dependents to queue
        let dependents = map_error(db.get_dependents(&current))?;
        to_uninstall.extend(dependents);

        // Perform actual uninstall
        map_error(bmm_lib::installer::uninstall_mod(PathBuf::from(
            mod_details.path,
        )))?;
        map_error(db.remove_installed_mod(&current))?;
    }

    Ok(())
}

#[tauri::command]
async fn remove_installed_mod(
    state: tauri::State<'_, AppState>,
    name: String,
    path: String,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;

    // Only check dependencies for framework mods
    let is_framework = name.to_lowercase() == "steamodded" || name.to_lowercase() == "talisman";

    if is_framework {
        let all_dependents = map_error(db.get_dependents(&name))?;

        // Filter out self-references
        let real_dependents: Vec<String> = all_dependents
            .into_iter()
            .filter(|dep| dep != &name)
            .collect();

        if !real_dependents.is_empty() {
            return Err(format!(
                "Use cascade_uninstall to remove {} with {} dependents",
                name,
                real_dependents.len()
            ));
        }
    }

    map_error(bmm_lib::installer::uninstall_mod(PathBuf::from(path)))?;
    map_error(db.remove_installed_mod(&name))
}

#[tauri::command]
async fn get_balatro_path(state: tauri::State<'_, AppState>) -> Result<Option<String>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    map_error(db.get_installation_path())
}

#[tauri::command]
async fn set_balatro_path(state: tauri::State<'_, AppState>, path: String) -> Result<(), String> {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return Err(e.to_string()),
    };
    map_error(db.set_installation_path(&path))
}

#[tauri::command]
async fn find_steam_balatro(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    let balatros = find_balatros();
    if let Some(path) = balatros.first() {
        let db = state.db.lock().map_err(|e| e.to_string())?;
        map_error(db.set_installation_path(&path.path.to_string_lossy()))?;
    }

    Ok(balatros
        .iter()
        .map(|b| b.path.to_string_lossy().into_owned())
        .collect())
}

#[tauri::command]
async fn get_steamodded_versions() -> Result<Vec<String>, String> {
    let installer = ModInstaller::new(ModType::Steamodded);
    installer
        .get_available_versions()
        .await
        .map(|versions| versions.into_iter().map(|v| v.to_string()).collect())
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn install_steamodded_version(version: String) -> Result<String, String> {
    let installer = ModInstaller::new(ModType::Steamodded);
    installer
        .install_version(&version)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_talisman_versions() -> Result<Vec<String>, String> {
    let installer = ModInstaller::new(ModType::Talisman);
    installer
        .get_available_versions()
        .await
        .map(|versions| versions.into_iter().map(|v| v.to_string()).collect())
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_latest_steamodded_release() -> Result<String, String> {
    // First try to get the version from cache
    if let Ok(Some(versions)) = cache::load_versions_cache("steamodded") {
        if !versions.is_empty() {
            // We have cached versions, use the first one (most recent)
            let version = &versions[0];
            return Ok(format!(
                "https://github.com/Steamodded/smods/archive/refs/tags/{}.zip",
                version
            ));
        }
    }

    // If cache miss or empty, fetch from network
    let installer = ModInstaller::new(ModType::Steamodded);
    installer
        .get_latest_release()
        .await
        .map(|version| {
            // Convert the version string to a proper GitHub download URL
            match installer.mod_type {
                ModType::Steamodded => {
                    format!(
                        "https://github.com/Steamodded/smods/archive/refs/tags/{}.zip",
                        version
                    )
                }
                // Fallback for other types if needed
                _ => format!(
                    "https://github.com/Steamodded/smods/archive/refs/tags/{}.zip",
                    version
                ),
            }
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn install_talisman_version(version: String) -> Result<String, String> {
    let installer = ModInstaller::new(ModType::Talisman);
    installer
        .install_version(&version)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_background_state(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    map_error(db.get_background_enabled())
}

#[tauri::command]
async fn set_background_state(
    state: tauri::State<'_, AppState>,
    enabled: bool,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    map_error(db.set_background_enabled(enabled))
}

#[tauri::command]
async fn verify_path_exists(path: String) -> bool {
    match std::fs::exists(PathBuf::from(path)) {
        Ok(exists) => exists,
        Err(e) => {
            log::error!("Failed to check path existence: {}", e);
            false
        }
    }
}

#[tauri::command]
async fn path_exists(path: String) -> Result<bool, String> {
    let path = PathBuf::from(path);
    Ok(path.exists())
}

#[tauri::command]
async fn check_custom_balatro(
    state: tauri::State<'_, AppState>,
    path: String,
) -> Result<bool, String> {
    let path = PathBuf::from(&path);
    let is_valid = bmm_lib::balamod::Balatro::from_custom_path(path.clone()).is_some();

    if is_valid {
        let db = state.db.lock().map_err(|e| e.to_string())?;
        map_error(db.set_installation_path(&path.to_string_lossy()))?;
    }

    Ok(is_valid)
}

// #[tauri::command]
// async fn get_installed_mods() -> Vec<String> {
//     bmm_lib::finder::get_installed_mods()
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Set up panic hook with proper logging
    panic::set_hook(Box::new(|panic_info| {
        log::error!("Application crashed: {:?}", panic_info);
    }));

    let result = tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            app.emit("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Initialize database with error handling
            let db = map_error(Database::new())?;

            let discord_rpc = DiscordRpcManager::new();

            // Get initial Discord RPC setting from database
            let discord_rpc_enabled = db.is_discord_rpc_enabled().unwrap_or(true);
            discord_rpc.set_enabled(discord_rpc_enabled);

            app.manage(AppState {
                db: Mutex::new(db),
                discord_rpc: Mutex::new(discord_rpc),
            });

            let app_dir = app
                .path()
                .app_data_dir()
                .map_err(|_| AppError::DirNotFound(PathBuf::from("app data directory")))?;

            std::fs::create_dir_all(&app_dir).map_err(|e| AppError::DirCreate {
                path: app_dir.clone(),
                source: e.to_string(),
            })?;

            #[cfg(debug_assertions)]
            if let Some(window) = app.get_webview_window("main") {
                window.open_devtools();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            find_steam_balatro,
            check_custom_balatro,
            check_existing_installation,
            get_balatro_path,
            set_balatro_path,
            launch_balatro,
            check_steam_running,
            check_balatro_running,
            get_installed_mods_from_db,
            install_mod,
            add_installed_mod,
            remove_installed_mod,
            get_steamodded_versions,
            install_steamodded_version,
            install_talisman_version,
            get_talisman_versions,
            verify_path_exists,
            path_exists,
            check_mod_installation,
            refresh_mods_folder,
            save_mods_cache,
            load_mods_cache,
            save_versions_cache,
            load_versions_cache,
            set_lovely_console_status,
            get_lovely_console_status,
            check_untracked_mods,
            clear_cache,
            cascade_uninstall,
            force_remove_mod,
            get_dependents,
            reindex_mods,
            get_background_state,
            set_background_state,
            get_last_fetched,
            update_last_fetched,
            get_repo_path,
            clone_repo,
            pull_repo,
            list_directories,
            read_json_file,
            read_text_file,
            // get_mod_timestamps,
            get_mod_thumbnail,
            get_discord_rpc_status,
            set_discord_rpc_status,
            get_latest_steamodded_release,
            set_discord_rpc_status
        ])
        .run(tauri::generate_context!());

    if let Err(e) = result {
        log::error!("Failed to run application: {}", e);
        std::process::exit(1);
    }
}
