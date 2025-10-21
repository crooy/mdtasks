#!/bin/bash

# mdtasks installer script
# Installs mdtasks CLI tool globally

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BINARY_NAME="mdtasks"
INSTALL_DIR="${HOME}/.local/bin"
CARGO_INSTALL_DIR="${HOME}/.cargo/bin"
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Functions
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if cargo is installed
check_cargo() {
    if ! command -v cargo &> /dev/null; then
        log_error "Cargo is not installed. Please install Rust first:"
        echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    log_success "Cargo is installed"
}

# Check if we're in the right directory
check_project() {
    if [[ ! -f "${PROJECT_DIR}/Cargo.toml" ]] || [[ ! -f "${PROJECT_DIR}/src/main.rs" ]]; then
        log_error "This doesn't appear to be the mdtasks project directory"
        log_error "Please run this script from the mdtasks project root or scripts/ directory"
        exit 1
    fi
    log_success "Found mdtasks project"
}

# Install mdtasks
install_mdtasks() {
    log_info "Installing mdtasks..."

    # Build using cargo
    cd "$PROJECT_DIR"

    if cargo build --release; then
        log_success "mdtasks built successfully"
    else
        log_error "Failed to build mdtasks"
        exit 1
    fi

    # Install to ~/.local/bin
    mkdir -p "$INSTALL_DIR"
    cp target/release/mdtasks "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/mdtasks"

    log_success "mdtasks installed to $INSTALL_DIR"
}

# Check if installation was successful
verify_installation() {
    if command -v "$BINARY_NAME" &> /dev/null; then
        local version=$($BINARY_NAME --version 2>/dev/null || echo "unknown")
        log_success "mdtasks is installed and available: $version"

        # Show where it's installed
        local binary_path=$(which "$BINARY_NAME")
        log_info "Installed at: $binary_path"

        # Test basic functionality
        log_info "Testing basic functionality..."
        if $BINARY_NAME --help &> /dev/null; then
            log_success "mdtasks is working correctly"
        else
            log_warning "mdtasks installed but may have issues"
        fi
    else
        log_error "mdtasks installation failed or is not in PATH"
        exit 1
    fi
}

# Add to PATH if needed
check_path() {
    local local_bin_in_path=false

    # Check if local bin is in PATH
    if [[ ":$PATH:" == *":${INSTALL_DIR}:"* ]]; then
        local_bin_in_path=true
    fi

    if [[ "$local_bin_in_path" == false ]]; then
        log_warning "~/.local/bin may not be in your PATH"
        log_info "Add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
        echo ""
        log_info "Then run: source ~/.bashrc (or ~/.zshrc)"
    fi
}

# Show usage information
show_usage() {
    log_success "Installation complete!"
    echo ""
    log_info "Usage examples:"
    echo "  $BINARY_NAME list                    # List all tasks"
    echo "  $BINARY_NAME add \"New task\"         # Add a new task"
    echo "  $BINARY_NAME start 1                 # Start working on task 1"
    echo "  $BINARY_NAME subtasks add 1 \"Item\"  # Add subtask"
    echo "  $BINARY_NAME subtasks list 1         # List subtasks"
    echo "  $BINARY_NAME done 1                  # Mark task as done"
    echo ""
    log_info "For more help: $BINARY_NAME --help"
}

# Uninstall function
uninstall() {
    log_info "Uninstalling mdtasks..."

    # Remove from ~/.local/bin
    if [[ -f "$INSTALL_DIR/mdtasks" ]]; then
        rm "$INSTALL_DIR/mdtasks"
        log_success "mdtasks removed from $INSTALL_DIR"
    else
        log_warning "mdtasks not found in $INSTALL_DIR"
    fi

    # Also try cargo uninstall in case it was installed via cargo
    if cargo uninstall mdtasks 2>/dev/null; then
        log_success "mdtasks also removed from cargo installation"
    fi
}

# Main function
main() {
    echo "🚀 mdtasks installer"
    echo "==================="
    echo ""

    # Handle uninstall flag
    if [[ "$1" == "--uninstall" ]] || [[ "$1" == "-u" ]]; then
        uninstall
        exit 0
    fi

    # Handle help flag
    if [[ "$1" == "--help" ]] || [[ "$1" == "-h" ]]; then
        echo "Usage: $0 [options]"
        echo ""
        echo "Options:"
        echo "  --uninstall, -u    Uninstall mdtasks"
        echo "  --help, -h         Show this help message"
        echo ""
        exit 0
    fi

    # Installation process
    check_cargo
    check_project
    install_mdtasks
    verify_installation
    check_path
    show_usage
}

# Run main function with all arguments
main "$@"
