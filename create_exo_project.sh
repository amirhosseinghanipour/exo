#!/bin/bash

# --- Configuration ---
PROJECT_NAME="Exo"

# --- Safety Check ---
if [ -d "$PROJECT_NAME" ]; then
  echo "Error: Directory '$PROJECT_NAME' already exists. Please remove it or choose a different location."
  exit 1
fi

echo "Creating project structure for '$PROJECT_NAME'..."

# --- Create Root Project Directory and Basic Files ---
mkdir "$PROJECT_NAME"
cd "$PROJECT_NAME" || exit 1 # Exit if cd fails

touch Cargo.toml README.md LICENSE .gitignore rust-toolchain.toml
echo "Created root files."

# --- Create Top-Level Directories ---
mkdir assets config docs scripts crates tests
echo "Created top-level directories."

# --- Populate assets ---
mkdir -p assets/icons assets/default_pages
touch assets/icons/.gitkeep assets/default_pages/.gitkeep # .gitkeep to track empty dirs in git
echo "Created assets structure."

# --- Populate config ---
touch config/default_settings.toml
echo "Created config structure."

# --- Populate docs ---
touch docs/.gitkeep
echo "Created docs structure."

# --- Populate scripts ---
touch scripts/setup_dev_env.sh
chmod +x scripts/setup_dev_env.sh # Make setup script executable
echo "Created scripts structure."

# --- Populate tests ---
mkdir -p tests/e2e tests/integration
touch tests/e2e/.gitkeep tests/integration/.gitkeep
echo "Created tests structure."

# --- Populate crates ---
echo "Creating crates structure..."
CRATES_DIR="crates"

# Helper function to create a basic crate structure
create_crate() {
  local crate_name=$1
  local crate_type=${2:-lib} # Default to library crate

  mkdir -p "${CRATES_DIR}/${crate_name}/src"
  touch "${CRATES_DIR}/${crate_name}/Cargo.toml"
  if [ "$crate_type" == "bin" ]; then
    touch "${CRATES_DIR}/${crate_name}/src/main.rs"
  else
    touch "${CRATES_DIR}/${crate_name}/src/lib.rs"
  fi
  echo "  - Created crate: $crate_name"
}

# Create individual crates
create_crate app_shell bin
mkdir -p "${CRATES_DIR}/app_shell/src/ui" "${CRATES_DIR}/app_shell/src/platform"
touch "${CRATES_DIR}/app_shell/src/ui/mod.rs" \
      "${CRATES_DIR}/app_shell/src/ui/main_window.rs" \
      "${CRATES_DIR}/app_shell/src/ui/tab_strip.rs" \
      "${CRATES_DIR}/app_shell/src/ui/address_bar.rs"
touch "${CRATES_DIR}/app_shell/src/platform/mod.rs" \
      "${CRATES_DIR}/app_shell/src/platform/macos.rs" \
      "${CRATES_DIR}/app_shell/src/platform/windows.rs" \
      "${CRATES_DIR}/app_shell/src/platform/linux.rs" # Added linux platform file

create_crate browser_core
touch "${CRATES_DIR}/browser_core/src/profile_manager.rs" \
      "${CRATES_DIR}/browser_core/src/tab_manager.rs" \
      "${CRATES_DIR}/browser_core/src/history.rs" \
      "${CRATES_DIR}/browser_core/src/bookmarks.rs" \
      "${CRATES_DIR}/browser_core/src/settings.rs"

create_crate rendering_engine
mkdir -p "${CRATES_DIR}/rendering_engine/src/html_parser" \
         "${CRATES_DIR}/rendering_engine/src/css_parser" \
         "${CRATES_DIR}/rendering_engine/src/dom" \
         "${CRATES_DIR}/rendering_engine/src/cssom" \
         "${CRATES_DIR}/rendering_engine/src/style" \
         "${CRATES_DIR}/rendering_engine/src/layout" \
         "${CRATES_DIR}/rendering_engine/src/painting" \
         "${CRATES_DIR}/rendering_engine/src/compositing" \
         "${CRATES_DIR}/rendering_engine/src/gpu_renderer"
touch "${CRATES_DIR}/rendering_engine/src/html_parser/.gitkeep" \
      "${CRATES_DIR}/rendering_engine/src/css_parser/.gitkeep" \
      "${CRATES_DIR}/rendering_engine/src/dom/.gitkeep" \
      "${CRATES_DIR}/rendering_engine/src/cssom/.gitkeep" \
      "${CRATES_DIR}/rendering_engine/src/style/.gitkeep" \
      "${CRATES_DIR}/rendering_engine/src/layout/.gitkeep" \
      "${CRATES_DIR}/rendering_engine/src/painting/.gitkeep" \
      "${CRATES_DIR}/rendering_engine/src/compositing/.gitkeep" \
      "${CRATES_DIR}/rendering_engine/src/gpu_renderer/.gitkeep"

create_crate js_engine
mkdir -p "${CRATES_DIR}/js_engine/src/bindings" "${CRATES_DIR}/js_engine/src/runtime"
touch "${CRATES_DIR}/js_engine/src/bindings/.gitkeep" \
      "${CRATES_DIR}/js_engine/src/runtime/.gitkeep"

create_crate networking
touch "${CRATES_DIR}/networking/src/http_client.rs" \
      "${CRATES_DIR}/networking/src/dns_resolver.rs" \
      "${CRATES_DIR}/networking/src/websocket.rs" \
      "${CRATES_DIR}/networking/src/request_interceptor.rs"

create_crate storage
touch "${CRATES_DIR}/storage/src/cookies.rs" \
      "${CRATES_DIR}/storage/src/local_storage.rs" \
      "${CRATES_DIR}/storage/src/indexed_db.rs" \
      "${CRATES_DIR}/storage/src/http_cache.rs"

create_crate privacy_features
mkdir -p "${CRATES_DIR}/privacy_features/src/tracker_blocker"
touch "${CRATES_DIR}/privacy_features/src/tracker_blocker/.gitkeep" \
      "${CRATES_DIR}/privacy_features/src/fingerprinting_mitigation.rs" \
      "${CRATES_DIR}/privacy_features/src/https_upgrader.rs" \
      "${CRATES_DIR}/privacy_features/src/third_party_cookie_policy.rs" \
      "${CRATES_DIR}/privacy_features/src/header_sanitizer.rs"

create_crate security_context
touch "${CRATES_DIR}/security_context/src/permissions_manager.rs" \
      "${CRATES_DIR}/security_context/src/sandboxing.rs" \
      "${CRATES_DIR}/security_context/src/content_security_policy.rs"

create_crate process_manager
mkdir -p "${CRATES_DIR}/process_manager/src/ipc"
touch "${CRATES_DIR}/process_manager/src/ipc/.gitkeep" \
      "${CRATES_DIR}/process_manager/src/renderer_process.rs"

create_crate dev_tools
mkdir -p "${CRATES_DIR}/dev_tools/src/protocol" "${CRATES_DIR}/dev_tools/src/inspector"
touch "${CRATES_DIR}/dev_tools/src/protocol/.gitkeep" \
      "${CRATES_DIR}/dev_tools/src/inspector/.gitkeep"

create_crate shared_types
touch "${CRATES_DIR}/shared_types/src/url.rs" \
      "${CRATES_DIR}/shared_types/src/errors.rs"

create_crate ui_toolkit

echo "Finished creating crates structure."

# --- Add basic content to root Cargo.toml (Workspace definition) ---
cat << EOF > Cargo.toml
[workspace]
resolver = "2" # Recommended for modern Rust
members = [
    "crates/app_shell",
    "crates/browser_core",
    "crates/rendering_engine",
    "crates/js_engine",
    "crates/networking",
    "crates/storage",
    "crates/privacy_features",
    "crates/security_context",
    "crates/process_manager",
    "crates/dev_tools",
    "crates/shared_types",
    "crates/ui_toolkit",
]

[profile.release]
lto = true
codegen-units = 1
panic = 'abort'

# Example of shared dependencies (add more as needed)
# [workspace.dependencies]
# tokio = { version = "1", features = ["full"] }
# log = "0.4"
# url = "2"

EOF
echo "Added basic workspace definition to root Cargo.toml."

# --- Add basic .gitignore ---
cat << EOF > .gitignore
# Rust / Cargo
target/
Cargo.lock

# IDE specific
.idea/
.vscode/
*.iml

# OS specific
.DS_Store
Thumbs.db

# Secrets / Sensitive Data (Examples - Adapt as needed!)
# *.pem
# *.key
# .env

# Build artifacts / Logs
*.log
*~
*.swp

EOF
echo "Added basic .gitignore file."

echo "-------------------------------------"
echo "Project structure for '$PROJECT_NAME' created successfully!"
echo "Next steps:"
echo "1. cd $PROJECT_NAME"
echo "2. Initialize Git: git init && git add . && git commit -m 'Initial project structure'"
echo "3. Populate Cargo.toml files within each crate with dependencies."
echo "4. Start implementing the code!"
echo "-------------------------------------"

exit 0
