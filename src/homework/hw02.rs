use std::io;

fn main() {
    println!("Введи перше число:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Не вдалося прочитати");
    let a: i32 = input1.trim().parse().expect("Це не число!");

    println!("Введи друге число:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Не вдалося прочитати");
    let b: i32 = input2.trim().parse().expect("Це не число!");

    let sum = a + b;
    println!("Сума: {}", sum);
}
