use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point, // верхня ліва
    b: Point, // нижня права
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut covered: HashSet<Point> = HashSet::new();

    for rect in xs {
        for x in rect.a.x..rect.b.x {
            for y in rect.a.y..rect.b.y {
                covered.insert(Point { x, y });
            }
        }
    }

    covered.len() as i32
}

// Тестові дані з прикладу
fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 3 },
            b: Point { x: 5, y: 9 },
        },
        Rectangle {
            a: Point { x: 4, y: 5 },
            b: Point { x: 14, y: 7 },
        },
        Rectangle {
            a: Point { x: 10, y: 2 },
            b: Point { x: 14, y: 10 },
        },
    ]
}

fn main() {
    let data = test_data();
    let area = area_occupied(&data);
    println!("Загальна зайнята площа: {}", area);
}
