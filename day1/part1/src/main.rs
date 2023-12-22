use std::fs;

fn main() {
    let file_path = "../../data/input1.txt";
    let input = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    let mut total = 0;
    for line in input.lines() {
        total += parse_numbers(line);
    }
    println!("The output is {}", total);
}

fn concat_digits(x: u32, y: u32) -> u32 {
    return x * 10 + y;
}

fn parse_numbers(input: &str) -> u32 {
    let mut first = 0;
    let mut last = 0;
    for c in input.chars() {
        if c.is_digit(10) {
            if first == 0 {
                first = c.to_digit(10).unwrap();
                last = first;
            }
            else {
                last = c.to_digit(10).unwrap();
            }
        }
    }
    return concat_digits(first, last); 
}