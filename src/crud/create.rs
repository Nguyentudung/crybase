use std::fs::OpenOptions;
use std::io::{self, Write};

use crate::models::record::Record;

pub fn create(records: &mut Vec<Record>, index: &mut u32, _path: &String) -> std::io::Result<()> {
    let mut content: String = String::new();

    print!(" - Nội dung bản ghi: ");

    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut content).unwrap();

    let record = Record {
        id: *index,
        content: content.clone(),
    };

    // Lưu tạm ở ram
    records.push(record);

    // Lưu vào ổ cứng
    let mut _file = OpenOptions::new().append(true).create(true).open(_path)?;

    writeln!(_file, "{}|{}", index, content)?;

    *index += 1;

    println!("");
    println!("\x1b[32mLưu bản ghi thành công!\x1b[0m");

    Ok(())
}
