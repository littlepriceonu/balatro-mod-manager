# [![Balatro Mod Manager](images/title.svg)](#)

The Balatro Mod Manager by _Skyline_.

Balatro Mod Manager is a standalone tool made for [Balatro](https://store.steampowered.com/app/2379780/Balatro/) that makes finding, downloading, and installing mods easy.


<p align="center">
    <img width="49%" alt="Home screen" src="images/cover_1.png"/>
    &nbsp;
    <img width="49%" alt="Start screen" src="images/cover_2.png"/>
</p>

<p align="center">
    <a href="https://star-history.com/#skyline69/balatro-mod-manager&Date">
        <picture>
            <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=skyline69/balatro-mod-manager&type=Date&theme=dark" />
            <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=skyline69/balatro-mod-manager&type=Date" />
            <img width="75%" alt="Star History Chart" src="https://api.star-history.com/svg?repos=skyline69/balatro-mod-manager&type=Date" />
        </picture>
    </a>
</p>


# [![Download](images/download.svg)](https://github.com/skyline69/balatro-mod-manager/releases/latest)

Balatro Mod Manager is currently available for Windows and macOS. The installer is standalone and does not require any external libraries.

[Download the Balatro Mod Manager installer here](https://github.com/skyline69/balatro-mod-manager/releases/latest).

Scroll down to find **▸Assets** and download the right version of the installer for your system.
- Windows: `Balatro.Mod.Manager_…_x64-setup.exe` or `Balatro.Mod.Manager_…_x64_en-US.msi`
- macOS: `Balatro.Mod.Manager_…_universal.dmg`


# [![Build](images/build.svg)](#build-prerequisites)

Alternatively, if you would prefer to build Balatro Mod Manager yourself instead of downloading the [prebuilt installer](https://github.com/skyline69/balatro-mod-manager/releases/latest), Balatro Mod Manager can be compiled from source using the instructions below.


## Build Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (for the backend)
- [Deno](https://deno.land/#installation) (for the frontend)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites#installing-the-tauri-cli)
- [Task](https://taskfile.dev/) (for running task commands)

## Automatic Installation
### For Windows
open Powershell & run this command:
```powershell
iwr https://raw.githubusercontent.com/skyline69/balatro-mod-manager/main/scripts/install.ps1 -useb | iex
```

### For macOS
run this command:
```bash
curl -sL https://raw.githubusercontent.com/skyline69/balatro-mod-manager/main/scripts/install.sh | bash
```

## Manual Installation

1. Clone the repository & install deno's dependencies:
    ```sh
    git clone https://github.com/skyline69/balatro-mod-manager.git
    cd balatro-mod-manager && deno install --allow-scripts
    ```

2. Install dependencies for the Svelte frontend:
    ```sh
    cd src-tauri
    deno task install
    ```

3. Install dependencies for the Rust backend:
    ```sh
    cargo build
    ```

## Running the Project

### Development Mode

To start the project in development mode, use the provided taskfile:

1. Run the debug target:
    ```sh
    task debug
    ```

### Production Mode

To build the project for production:

1. Build the release target:
    ```sh
    task release
    ```


The built application will be located in the `src-tauri/target/release` directory.

## Cleaning the Build

To clean the build files, use the provided taskfile:

1. Run the clean target:
    ```sh
    task clean
    ```

> Font by Daniel Linssen
