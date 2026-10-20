use std::fs;
use std::io::{self, Write};

use crate::models::record::Record;

pub fn update(records: &mut Vec<Record>, _path: &String) -> std::io::Result<()> {
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

        // Lưu tạm trong ram
        records[index].content = input.trim().to_string();
        // Lưu trong ổ cứng

        let mut _file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(_path)?;

        fs::write(_path, "")?;

        for i in 0..records.len() {
            writeln!(_file, "{}|{}", records[i].id, records[i].content)?;
        }

        println!("\x1b[32mLưu bản ghi thành công!\x1b[0m");
    }

    Ok(())
}
