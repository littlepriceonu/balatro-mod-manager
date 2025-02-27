use crate::cache::Mod;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModLinkResult {
    pub is_mod: bool,
    pub mod_name: String,
}

lazy_static! {
    // Pre-compile regex patterns for better performance
    static ref FILE_EXTENSION_REGEX: Regex = Regex::new(r"\.(txt|lua|json|md|png|jpg|jpeg|gif|mp3|ogg|wav)$").unwrap();
    static ref GITHUB_MOD_PATTERN1: Regex = Regex::new(r"github\.com/[^/]+/([^/]+)$").unwrap();
    static ref GITHUB_MOD_PATTERN2: Regex = Regex::new(r"github\.com/[^/]+/([^/]+)(/|/tree/main|/tree/master)?$").unwrap();
}

pub fn is_internal_mod_link(url: &str, mods_array: Vec<Mod>) -> ModLinkResult {
    // Early return for common exclusion patterns
    if FILE_EXTENSION_REGEX.is_match(url) || url.contains("/blob/") || url.contains("/tree/") {
        return ModLinkResult {
            is_mod: false,
            mod_name: String::new(),
        };
    }

    // Extract repository name using pre-compiled regex
    let repo_name = if let Some(captures) = GITHUB_MOD_PATTERN1.captures(url) {
        captures.get(1).map(|m| m.as_str().to_lowercase())
    } else if let Some(captures) = GITHUB_MOD_PATTERN2.captures(url) {
        captures.get(1).map(|m| m.as_str().to_lowercase())
    } else {
        None
    };

    // Early return if no repo name found
    let repo_name = match repo_name {
        Some(name) => name,
        None => return ModLinkResult { is_mod: false, mod_name: String::new() },
    };

    // Create lowercase variants of repo name once
    let repo_name_lower = repo_name.as_str();

    // Try to find a matching mod
    for mod_item in &mods_array {
        let title_lower = mod_item.title.to_lowercase();
        
        // Direct match on title
        if title_lower == repo_name_lower {
            return ModLinkResult {
                is_mod: true,
                mod_name: mod_item.title.clone(),
            };
        }
        
        // Match on repo URL
        if mod_item.repo.to_lowercase().contains(repo_name_lower) {
            return ModLinkResult {
                is_mod: true,
                mod_name: mod_item.title.clone(),
            };
        }
        
        // Match with spaces replaced or removed (compute only when needed)
        let title_dashes = title_lower.replace(' ', "-");
        if repo_name_lower == title_dashes {
            return ModLinkResult {
                is_mod: true,
                mod_name: mod_item.title.clone(),
            };
        }
        
        let title_underscores = title_lower.replace(' ', "_");
        if repo_name_lower == title_underscores {
            return ModLinkResult {
                is_mod: true,
                mod_name: mod_item.title.clone(),
            };
        }
        
        let title_no_spaces = title_lower.replace(' ', "");
        if repo_name_lower == title_no_spaces {
            return ModLinkResult {
                is_mod: true,
                mod_name: mod_item.title.clone(),
            };
        }
    }

    ModLinkResult {
        is_mod: false,
        mod_name: String::new(),
    }
}

