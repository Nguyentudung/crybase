use std::fs::{self, File};
use std::path::Path;

pub fn check_data_storage(_path: &String) -> std::io::Result<()> {
    fs::create_dir_all("cry_data")?;

    let _path = Path::new(_path);

    if !_path.exists() {
        File::create(_path)?;
        println!("Đã tạo database mới!");
    } else {
        println!("Database đã tồn tại!");
    }

    Ok(())
}
