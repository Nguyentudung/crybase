use std::io::{self, Write};

use crate::models::record::Record;

pub fn create(records: &mut Vec<Record>, index: &mut u32) {
    let mut content: String = String::new();

    print!(" - Nội dung bản ghi: ");

    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut content).unwrap();

    let record = Record {
        id: *index,
        content: content,
    };

    records.push(record);
    *index += 1;

    println!("");
    println!("\x1b[32mLưu bản ghi thành công!\x1b[0m");
}
