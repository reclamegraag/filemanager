mod commands;
mod fs;

use commands::{
    filesystem::{read_directory, get_parent_directory, get_home_directory, open_file},
    config::{load_config, save_config},
    wsl::{get_wsl_distros, wsl_copy},
    operations::{copy_files, move_files, delete_files, create_directory, rename_file, get_file_info},
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // Filesystem
            read_directory,
            get_parent_directory,
            get_home_directory,
            open_file,
            get_file_info,
            // Config
            load_config,
            save_config,
            // WSL
            get_wsl_distros,
            wsl_copy,
            // File operations
            copy_files,
            move_files,
            delete_files,
            create_directory,
            rename_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
