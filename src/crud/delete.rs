use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Write};

use crate::models::record::Record;

pub fn delete(records: &mut Vec<Record>, _path: &String) -> std::io::Result<()> {
    let mut input: String = String::new();

    print!(" - Nhập id bản ghi: ");

    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();

    let target_id: u32 = input.trim().parse().unwrap();

    let index: Option<usize> = records.iter().position(|records| records.id == target_id);

    if let Some(index) = index {
        // Lưu tạm ở ram
        records.remove(index);
        // Lưu vào ổ cứng
        let mut _file = OpenOptions::new().append(true).create(true).open(_path)?;

        fs::write(_path, "")?;

        for i in 0..records.len() {
            writeln!(_file, "{}|{}", records[i].id, records[i].content)?;
        }

        println!(
            "\x1b[32mĐã xáo bản ghi có id {} thành công!\x1b[0m",
            target_id
        )
    } else {
        println!("\x1b[33mKhông tìm thấy bản ghi có id {}\x1b[0m", target_id);
    }

    Ok(())
}
