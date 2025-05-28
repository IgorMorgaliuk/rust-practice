const W: usize = 40;
const H: usize = 20;

fn main() {
    let mut result = String::new();

    for y in 0..H {
        for x in 0..W {
            let mut ch = ' ';

            if y == 0 {
                ch = '*';
            } else if y == H - 1 {
                ch = '*';
            } else if x == 0 {
                ch = '*';
            } else if x == W - 1 {
                ch = '*';
            }

            let mut i = 0;
            while i < H {
                let diag_x = i * (W - 1) / (H - 1);
                if y == i && x == diag_x {
                    ch = '*';
                }
                i += 1;
            }

            let mut j = 0;
            while j < H {
                let diag_x = (H - 1 - j) * (W - 1) / (H - 1);
                if y == j && x == diag_x {
                    ch = '*';
                }
                j += 1;
            }

            result.push(ch);
        }
        result.push('\n');
    }

    print!("{}", result);
}
