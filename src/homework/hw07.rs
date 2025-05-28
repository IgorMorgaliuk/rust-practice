pub fn swap_case(s: &str) -> String {
    let mut result = String::new();
    let characters: Vec<char> = s.chars().collect();

    for i in 0..characters.len() {
        let ch = characters[i];

        let code = ch as u32;

        if (code >= 65 && code <= 90) {
            // A-Z → a-z
            let lower = (code + 32) as u8 as char;
            result.push(lower);
        } else if (code >= 97 && code <= 122) {
            // a-z → A-Z
            let upper = (code - 32) as u8 as char;
            result.push(upper);
        } else {
            let unchanged = format!("{}", ch);
            for c in unchanged.chars() {
                result.push(c);
            }
        }
    }

    return result; 
}

// Тест
fn main() {
    let mut input = String::from("Hello, Rust!");
    let output = swap_case(&input);
    println!("Original: {}", input);
    println!("Swapped : {}", output);
}
