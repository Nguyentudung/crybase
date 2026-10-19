use std::fs;

use crate::models::record::Record;

pub fn load_record(
    records: &mut Vec<Record>,
    max_id: &mut u32,
    _path: &String,
) -> std::io::Result<()> {
    let data: String = fs::read_to_string(_path)?;

    for line in data.lines() {
        if line != "" {
            let mut parts = line.split("|");

            let id = parts.next().unwrap();
            let content = parts.next().unwrap_or("");

            let id: u32 = id.parse().expect("ID không hợp lệ");

            let record: Record = Record {
                id: id,
                content: content.to_string(),
            };

            records.push(record);

            if id >= *max_id {
                *max_id = id + 1;
            }
        }
    }

    Ok(())
}
