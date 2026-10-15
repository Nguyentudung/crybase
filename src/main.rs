use std::io::{self, Write};

mod models;
use models::record::Record;

mod crud;
use crud::create::create;
use crud::delete::delete;
use crud::read::read;
use crud::update::update;

mod list;
use list::list;

fn main() {
    let mut records: Vec<Record> = Vec::new();
    let mut index: u32 = 1;
    let mut command = String::new();

    while command.trim() != "exit" {
        print!("\x1b[34mcrydb > \x1b[0m");

        io::stdout().flush().unwrap();

        command.clear();

        io::stdin()
            .read_line(&mut command)
            .expect("Không đọc được lệnh!");

        if command.trim() == "create" {
            println!("");
            println!(" - id bản ghi: {}", index);
            create(&mut records, &mut index);
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
}
