fn error(line: i32, message: String, has_error: bool) {
    report(line, "", message, has_error);
}

fn report(line: i32, location: &str, message: String, mut has_error: bool) {
    eprintln!("[line {line} ] Error {location}: {message}");
    has_error = true;
}