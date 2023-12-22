use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path = "../../data/input1.txt";
    let input = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let numbers = make_hashmap();

    let mut total = 0;
    for line in input.lines() {
        total += parse_numbers(line, numbers.clone());
    }
    println!("The output is {}", total);
}

fn make_hashmap() -> HashMap<String, u32> {
    let mut numbers = HashMap::new();
    numbers.insert("one".to_string(), 1);
    numbers.insert("two".to_string(), 2);
    numbers.insert("three".to_string(), 3);
    numbers.insert("four".to_string(), 4);
    numbers.insert("five".to_string(), 5);
    numbers.insert("six".to_string(), 6);
    numbers.insert("seven".to_string(), 7);
    numbers.insert("eight".to_string(), 8);
    numbers.insert("nine".to_string(), 9);
    return numbers;
}

fn concat_digits(x: u32, y: u32) -> u32 {
    return x * 10 + y;
}

fn parse_numbers(input: &str, numbers: HashMap<String, u32>) -> u32 {
    let mut first = 0;
    let mut last = 0;
    let mut buffer = String::new();
    for c in input.chars() {
        if c.is_digit(10) {
            buffer.clear();
            if first == 0 {
                first = c.to_digit(10).unwrap();
                last = first;
            }
            else {
                last = c.to_digit(10).unwrap();
            }
        }
        else {
            buffer.push(c);
            //println!("{}", buffer);
            let mut flag = false;
            for (key, value) in &numbers {
                if key.starts_with(&buffer) {
                    flag = true;
                    if buffer == *key {
                        if first == 0 {
                            first = *value;
                            last = first;
                        }
                        else {
                            last = *value;
                        }
                        buffer.clear();
                        buffer.push(c);
                    }
                }
            }
            if !flag {
                buffer.remove(0);
            }
        }
    }
    println!("{} {}", first, last);
    return concat_digits(first, last); 
}