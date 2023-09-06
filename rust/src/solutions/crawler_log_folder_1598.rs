pub fn min_operations(logs: Vec<String>) -> i32 {
    let mut level = 0;

    for cmd in logs {
        match cmd.as_str() {
            "../" => if level > 0 { level -= 1 }
            "./" => {}
            _ => level += 1
        }
    }

    level
}
