const WIDTH: usize = 9;
const HEIGHT: usize = 9;

fn main() {
    let mut output = String::new();
    let half = HEIGHT / 2;

    for i in 0..=half {
        for _ in 0..(half - i) {
            output.push(' ');
        }
        for _ in 0..(2 * i + 1) {
            output.push('*');
        }
        output.push('\n');
    }

    for i in (0..half).rev() {
        for _ in 0..(half - i) {
            output.push(' ');
        }
        for _ in 0..(2 * i + 1) {
            output.push('*');
        }
        output.push('\n');
    }

    print!("{}", output);
}
