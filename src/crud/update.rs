use std::io::{self, Write};

use crate::models::record::Record;

pub fn update(records: &mut Vec<Record>) {
    let mut input: String = String::new();

    print!(" - Nhập id bản ghi: ");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let target_id: u32 = input.trim().parse().unwrap();

    let index: Option<usize> = records.iter().position(|records| records.id == target_id);

    if let Some(index) = index {
        print!(" - Nội dung bản ghi: ");

        input.clear();

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        records[index].content = input.trim().to_string();
        println!("\x1b[32mLưu bản ghi thành công!\x1b[0m");
    }
}
