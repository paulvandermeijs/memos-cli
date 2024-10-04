use std::io::Read;

pub(crate) fn get_stdin() -> Option<String> {
    use std::io::IsTerminal;

    let stdin = std::io::stdin();

    if stdin.is_terminal() {
        return None;
    }

    let mut buffer = String::new();

    if let Err(_) = stdin.lock().read_to_string(&mut buffer) {
        return None;
    }

    let buffer = buffer.trim().to_string();

    if buffer.is_empty() {
        return None;
    }

    Some(buffer)
}
