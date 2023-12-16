use std::path::PathBuf;

pub fn get_test_file_path(file_name: &str) -> PathBuf {
    let test_image_dir = project_root::get_project_root()
        .expect("Can not find project root")
        .join("tests")
        .join("images");

    test_image_dir.join(file_name)
    // .canonicalize()
    // .expect(&format!(
    //     "Can not create path for {:?} and {}",
    //     test_image_dir, file_name
    // ))
}
