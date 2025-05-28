fn main() {
    let a = 4;
    let mut res = String::new();
    let mut j = 1;

    while j <= a {
        let h = j + 1;
        let mut i = 0;

        while i < h {
            let mut k = 0;
            let mut s = String::new();

            while k < (a * 2 - i) {
                s.push(' ');
                k += 1;
            }

            let mut l = 0;
            while l < (2 * i + 1) {
                s.push('*');
                l += 1;
            }

            res.push_str(&s);
            res.push('\n');
            i += 1;
        }

        j += 1;
    }

    println!("{}", res);
}