#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Balatro Mod Manager Builder${NC}"
echo "----------------------------------------"
echo "Build started at $(date)"

# Check if running on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo -e "${RED}Error: This builder is for macOS only.${NC}"
    exit 1
fi

# Check for required tools
echo "Checking dependencies..."

if ! command -v git &> /dev/null; then
    echo -e "${RED}Error: git not found. Please install git first.${NC}"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo not found. Please install Rust/Cargo first.${NC}"
    exit 1
fi

if ! command -v deno &> /dev/null; then
    echo -e "${RED}Error: deno not found. Please install deno first.${NC}"
    exit 1
fi

# Create a temporary directory
BUILD_DIR=$(mktemp -d)
echo -e "${YELLOW}Creating temporary build directory: ${BUILD_DIR}${NC}"

# Clone the repository
echo -e "${YELLOW}1. Cloning repository...${NC}"
git clone https://github.com/skyline69/balatro-mod-manager.git "${BUILD_DIR}/balatro-mod-manager" || { 
    echo -e "${RED}Git clone failed${NC}"
    rm -rf "${BUILD_DIR}"
    exit 1
}

# Change to repository directory
cd "${BUILD_DIR}/balatro-mod-manager" || {
    echo -e "${RED}Failed to enter repository directory${NC}"
    rm -rf "${BUILD_DIR}"
    exit 1
}

# Build steps
echo -e "${YELLOW}2. Installing deno dependencies...${NC}"
deno install --allow-scripts || {
    echo -e "${RED}deno install failed${NC}"
    rm -rf "${BUILD_DIR}"
    exit 1
}

echo -e "${YELLOW}3. Building frontend...${NC}"
deno task build || {
    echo -e "${RED}Frontend build failed${NC}"
    rm -rf "${BUILD_DIR}"
    exit 1
}

echo -e "${YELLOW}4. Building Rust backend...${NC}"
cd src-tauri && SKIP_BUILD_SCRIPT=1 cargo build --release || {
    echo -e "${RED}Cargo build failed${NC}"
    rm -rf "${BUILD_DIR}"
    exit 1
}

cd ..

echo -e "${YELLOW}5. Creating app bundle...${NC}"
cargo tauri build || {
    echo -e "${RED}Tauri build failed${NC}"
    rm -rf "${BUILD_DIR}"
    exit 1
}

# Cleanup
echo -e "${YELLOW}6. Cleaning up...${NC}"
rm -rf "${BUILD_DIR}"

echo -e "${GREEN}Installation completed successfully!${NC}"
echo
echo -e "${YELLOW}Note: Security dialog might appear on first launch${NC}"

exit 0
