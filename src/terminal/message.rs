use super::status::Status;

pub fn print_message(message: &str, status: &Status) {
    match status {
        Status::Success => {
            println!("\x1b[32m{}\x1b[0m", message);
        }
        Status::Warning => {
            println!("\x1b[33m{}\x1b[0m", message);
        }
        Status::Error => {
            println!("\x1b[31m{}\x1b[0m", message);
        }
    }
}
