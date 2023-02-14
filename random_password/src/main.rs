use rand::Rng;

fn generate_password(length: usize) -> String {
    let chars: Vec<char> = (0..length)
        .map(|_| {
            let char_type = rand::thread_rng().gen_range(0..3);
            match char_type {
                0 => rand::thread_rng().gen_range(b'A'..=b'Z') as char, // uppercase letter
                1 => rand::thread_rng().gen_range(b'a'..=b'z') as char, // lowercase letter
                2 => rand::thread_rng().gen_range(b'0'..=b'9') as char, // digit
                _ => rand::thread_rng().gen_range(b'!'..=b'~') as char, // special character
            }
        })
        .collect();

    chars.iter().collect()
}

fn main() {
    let length = loop {
        let mut input = String::new();
        println!("Enter password length (8-64): ");
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<usize>() {
            Ok(length) if length >= 8 && length <= 64 => break length,
            _ => println!("Invalid length. Please enter a number between 8 and 64."),
        }
    };

    let password = generate_password(length);
    println!("Your password is: {}", password);
}
