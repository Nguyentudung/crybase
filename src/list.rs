use crate::models::record::Record;

pub fn list(records: &Vec<Record>) {
    if records.is_empty() {
        println!("\x1b[33mDanh sách trống!\x1b[0m")
    } else {
        for i in 0..records.len() {
            println!("id: {}, nội dung: {}", records[i].id, records[i].content)
        }
    }
}
