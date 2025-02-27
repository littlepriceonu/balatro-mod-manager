# Define OS-specific variables
CLEAR_SCREEN := if os() == "windows" { "cls" } else { "clear" }
MACOS_TARGET_ENV := if os() != "windows" { "MACOSX_DEPLOYMENT_TARGET=11.0" } else { "" }
TARGET := if os() == "windows" { "x86_64-pc-windows-msvc" } else { "universal-apple-darwin" }

# Debug target
debug:
    {{CLEAR_SCREEN}}
    @echo ""
    cargo tauri dev

# Platform-specific release targets
release-macos:
    {{CLEAR_SCREEN}}
    @echo ""
    {{MACOS_TARGET_ENV}} cargo tauri build --target universal-apple-darwin --verbose

release-windows:
    {{CLEAR_SCREEN}}
    @echo ""
    cargo tauri build --target x86_64-pc-windows-msvc --verbose

release-macos-production:
    @echo ""
    {{MACOS_TARGET_ENV}} APPLE_SIGNING_IDENTITY="Developer ID Application: Öner Efe Dasguney (C4G7YDX6RS)" cargo tauri build --target universal-apple-darwin --verbose

# Default release target
release:
    {{CLEAR_SCREEN}}
    @echo ""
    {{MACOS_TARGET_ENV}} cargo tauri build --target {{TARGET}} --verbose

# Clean target
clean:
    echo "Cleaning all build files..."
    cd src-tauri
    cargo clean

