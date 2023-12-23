use std::fs;

fn main() {
    let file_path = "../../data/input2.txt";
    let input = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let limit = 14;

    let mut total = 0;
    for line in input.lines() {
        let sets: Vec<String> = line.split(';').map(String::from).collect();
        let index = sets[0].find(':').unwrap();
        let n = sets[0][5..index].parse::<u32>().unwrap();
        if parse_sets(sets, limit) {
            total += n;
            println!("Game id {} is valid", n);
        }
    }
    println!("The output is {}", total);
}

fn parse_sets(sets: Vec<String>, limit: u32) -> bool {
    for set in sets {
        let mut nbuffer = 0;
        let draw: Vec<String> = set.split(' ').map(String::from).collect();
        for item in draw {
            let item = item.trim_end_matches(',');
            let result = item.parse::<u32>();
            match result {
                Ok(number) => {
                    nbuffer = number;
                }

                Err(_) => {
                    if item == "red" {
                        nbuffer += 2;
                    }
                    else if item == "green" {
                        nbuffer += 1;
                    }
                    if nbuffer > limit {
                        println!("{}", nbuffer);
                        return false;
                    }
                    else {
                        nbuffer = 0;
                    }
                }
            }
        }
    }
    return true;
}