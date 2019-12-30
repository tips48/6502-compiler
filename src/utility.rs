pub fn append_tabs(amount: u8) -> String {
    let mut result = String::new();

    for _ in 0..amount {
        result.push(' ');
    }

    result
}
