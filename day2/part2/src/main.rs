use std::fs;

fn main() {
    let file_path = "../../data/input2.txt";
    let input = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let mut total = 0;
    for line in input.lines() {
        let sets: Vec<String> = line.split(';').map(String::from).collect();
        total += parse_sets(sets);
    }
    println!("The output is {}", total);
}

fn parse_sets(sets: Vec<String>) -> u32 {
    let (mut rmax, mut gmax, mut bmax) = (0, 0, 0);
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
                        if nbuffer > rmax {
                            rmax = nbuffer;
                        }
                    }
                    else if item == "green" {
                        if nbuffer > gmax {
                            gmax = nbuffer;
                        }
                    }
                    else if item == "blue" {
                        if nbuffer > bmax {
                            bmax = nbuffer;
                        }
                    }
                    nbuffer = 0;
                }
            }
        }
    }
    return rmax * gmax * bmax;
}