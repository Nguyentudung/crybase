use std::io::{self, Write};

use crybase::models::record::Record;

use crybase::crud::create::create;
use crybase::crud::delete::delete;
use crybase::crud::list::list;
use crybase::crud::read::read;
use crybase::crud::update::update;

use crybase::db::connect::init_database;
use crybase::db::records::get_all_records;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _path = String::from("cry_data/main.db");
    let mut command = String::new();

    // Khởi tạo database và lấy connection pool
    let pool = init_database(&_path).await?;

    // Tải dữ liệu từ SQLite vào bộ nhớ
    let records: Vec<Record> = get_all_records(&pool).await?;

    println!("Loaded {} records", records.len());

    loop {
        print!("\x1b[34mcrybase > \x1b[0m");
        io::stdout().flush()?;

        command.clear();
        io::stdin().read_line(&mut command)?;

        match command.trim() {
            "create" => {
                println!();
                // Chuyển create sang sử dụng &pool
                println!();
            }
            "read" => {
                println!();
                read(&records);
                println!();
            }
            "update" => {
                println!();
                // Chuyển update sang sử dụng &pool
                println!();
            }
            "delete" => {
                println!();
                // Chuyển delete sang sử dụng &pool
                println!();
            }
            "list" => {
                println!();
                list(&records);
                println!();
            }
            "exit" => {
                println!("\nTạm biệt\n");
                break;
            }
            "" => {}
            _ => println!("\nLệnh không tồn tại!\n"),
        }
    }

    Ok(())
}
