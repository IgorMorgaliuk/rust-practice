fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec![String::from("")];
    }

    let prev = gray(n - 1); 
    let mut result = Vec::new();

    for s in &prev {
        result.push(format!("0{}", s));
    }

    for s in prev.iter().rev() {
        result.push(format!("1{}", s));
    }

    result
}

#[test]
fn test() {
    let test_data = [
        (0, vec![""]),
        (1, vec!["0", "1"]),
        (2, vec!["00", "01", "11", "10"]),
        (3, vec!["000", "001", "011", "010", "110", "111", "101", "100"]),
        (4, vec![
            "0000", "0001", "0011", "0010",
            "0110", "0111", "0101", "0100",
            "1100", "1101", "1111", "1110",
            "1010", "1011", "1001", "1000",
        ]),
    ];

    test_data.iter().for_each(|(n, expected)| {
        let output = gray(*n);
        let output_strs: Vec<&str> = output.iter().map(|s| s.as_str()).collect();
        assert_eq!(output_strs, *expected);
    });
}
