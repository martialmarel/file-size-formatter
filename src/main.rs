use file_size_formatter::process;

fn main() {
    match process() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
