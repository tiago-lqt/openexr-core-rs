pub mod errors;

use std::path::PathBuf;

pub fn get_test_image_dir() -> PathBuf {
    project_root::get_project_root()
        .expect("Can not find project root")
        .join("tests/test_data/images")
}

pub fn get_test_file_path(file_name: &str) -> PathBuf {
    let test_image_dir = get_test_image_dir();
    test_image_dir.join(file_name)
}

pub fn get_temp_dir() -> PathBuf {
    std::env::temp_dir()
}
