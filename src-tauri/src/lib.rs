mod commands;
mod fs;
mod indexer;

use commands::{
    config::{load_config, save_config},
    filesystem::{get_home_directory, get_parent_directory, open_file, read_directory},
    indexer::{
        clear_index_cache, get_index_status, search_index, start_indexing, stop_indexing,
        IndexerState,
    },
    operations::{
        copy_files, create_directory, delete_files, get_file_info, move_files, rename_file,
    },
    search::{get_available_drives, search_files},
    wsl::{get_wsl_distros, wsl_copy},
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .manage(IndexerState::new())
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
            // Search
            search_files,
            get_available_drives,
            // Indexer
            start_indexing,
            search_index,
            get_index_status,
            stop_indexing,
            clear_index_cache,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
