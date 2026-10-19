use std::io::{self, Write};

mod models;
use models::record::Record;

mod crud;
use crud::create::create;
use crud::delete::delete;
use crud::list::list;
use crud::read::read;
use crud::update::update;

mod storage;
use storage::file::check_data_storage;
use storage::load::load_record;

fn main() -> std::io::Result<()> {
    let _path: String = String::from("cry_data/main.db");
    let mut records: Vec<Record> = Vec::new();
    let mut id: u32 = 1;
    let mut command = String::new();

    // Kiểm tra và khởi tạo thư mục lưu trữ dữ liệu
    check_data_storage(&_path)?;
    // Load dữ liệu vào vec
    load_record(&mut records, &mut id, &_path)?;

    while command.trim() != "exit" {
        print!("\x1b[34mcrydb > \x1b[0m");

        io::stdout().flush().unwrap();

        command.clear();

        io::stdin()
            .read_line(&mut command)
            .expect("Không đọc được lệnh!");

        if command.trim() == "create" {
            println!("");
            println!(" - id bản ghi: {}", id);
            create(&mut records, &mut id, &_path)?;
            println!("");
        } else if command.trim() == "read" {
            println!("");
            read(&records);
            println!("");
        } else if command.trim() == "update" {
            println!("");
            update(&mut records);
            println!("");
        } else if command.trim() == "delete" {
            println!("");
            delete(&mut records);
            println!("");
        } else if command.trim() == "list" {
            println!("");
            list(&records);
            println!("");
        } else if command.trim() == "exit" {
            println!("\nTạm biệt\n");
        } else {
            println!("\nLệnh không tồn tại!\n")
        }
    }

    Ok(())
}
