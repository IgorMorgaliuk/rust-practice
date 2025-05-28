pub fn is_palindrome(x: u32) -> bool {
    let number_string = format!("{}", x);
    let mut chars_vec: Vec<char> = Vec::new();
    let mut reversed_vec: Vec<char> = Vec::new();

    for ch in number_string.chars() {
        chars_vec.push(ch);
    }

    let mut i = chars_vec.len();
    while i > 0 {
        i -= 1;
        reversed_vec.push(chars_vec[i]);
    }

    let mut is_same = true;
    let mut index = 0;
    while index < chars_vec.len() {
        if chars_vec[index] != reversed_vec[index] {
            is_same = false;
            break;
        }
        index += 1;
    }

    return is_same;
}
