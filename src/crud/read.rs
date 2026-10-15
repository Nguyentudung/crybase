use std::io::{self, Write};

use crate::models::record::Record;

pub fn read(records: &Vec<Record>) {
    let mut input: String = String::new();

    print!(" - Nhập id bản ghi: ");

    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();

    let target_id: u32 = input.trim().parse().unwrap();

    let index: Option<usize> = records.iter().position(|record| record.id == target_id);

    if let Some(index) = index {
        println!("Nội dùng bản ghi: {}", records[index].content);
    } else {
        println!("\x1b[33mKhông tìm thấy bản ghi!\x1b[0m");
    }
}
