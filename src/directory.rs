use std::path::PathBuf;

pub fn init_fs() -> std::io::Result<()> {
    let data_dir = get_data_dir();
    std::fs::create_dir_all(data_dir)
}

pub fn get_data_dir() -> PathBuf {
    directories::ProjectDirs::from("com", "sfrem", "finder")
        .unwrap()
        .data_dir()
        .to_path_buf()
}
