use std::fs;
use std::io::Write;

pub fn log(msg: &str) {

    let mut log_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("/home/neto/documents/scripts/rust/game_of_life/log.txt")
        .unwrap();

    write!(log_file, "{}\n", msg);
}

pub fn clear() {
    let mut log_file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("/home/neto/documents/scripts/rust/game_of_life/log.txt")
        .unwrap();

    writeln!(log_file, "-- Log File --");   
}
