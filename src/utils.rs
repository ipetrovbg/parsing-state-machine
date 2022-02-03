const MAX_LEN: usize = 60;
pub fn print_wrap(message: &str, c: char) {
    let message_len = message.len();
    let pretty_str = generate_pretty(message_len, c);
    if message_len % 2 != 0 {
        println!("{} {} {}", pretty_str, message.to_uppercase(), pretty_str);
    } else {
        let mut pretty_str_right = pretty_str.clone();
        pretty_str_right.pop();
        println!("{} {} {}", pretty_str, message.to_uppercase(), pretty_str_right);
    }
}

fn generate_pretty(len: usize, c: char) -> String {
    let mut pretty_string = String::new();
    let mut i = 0;
    let max = MAX_LEN - len;
    if (max / 2) <= 0 {
        return pretty_string.clone();
    }

    while i < (MAX_LEN - len) / 2 {
        pretty_string.push(c);
        i = i + 1;
    }

    pretty_string
}