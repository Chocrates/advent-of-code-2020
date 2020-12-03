use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn one() {
    let lines = lines_from_file("input/1.txt");
    for line1 in &lines {
        for line2 in &lines {
            if *line1 == *line2 {
                continue;
            }
            if (*line1).parse::<i32>().unwrap() + (*line2).parse::<i32>().unwrap() == 2020 {
                println!(
                    "{} {} {} {}",
                    *line1,
                    *line2,
                    (*line1).parse::<i32>().unwrap() + (*line2).parse::<i32>().unwrap(),
                    (*line1).parse::<i32>().unwrap() * (*line2).parse::<i32>().unwrap()
                );
                break;
            }
        }
    }
}

pub fn one_two() {
    let lines = lines_from_file("input/1.txt");
    for line1 in &lines {
        for line2 in &lines {
            for line3 in &lines {
                if (*line1).parse::<i32>().unwrap()
                    + (*line2).parse::<i32>().unwrap()
                    + (*line3).parse::<i32>().unwrap()
                    == 2020
                {
                    println!(
                        "{} {} {} {} {}",
                        *line1,
                        *line2,
                        *line3,
                        (*line1).parse::<i32>().unwrap()
                            + (*line2).parse::<i32>().unwrap()
                            + (*line3).parse::<i32>().unwrap(),
                        (*line1).parse::<i32>().unwrap()
                            * (*line2).parse::<i32>().unwrap()
                            * (*line3).parse::<i32>().unwrap()
                    );
                    break;
                }
            }
        }
    }
}

pub fn two() {
    // read lines from file
    // for each line parse rules
    // count correct passwords

    let lines = lines_from_file("input/2.txt");
    let mut good_passwords = 0;

    for line in &lines {
        let rules = line.split(":").nth(0).unwrap();
        let range_rules = rules.split(" ").nth(0).unwrap();
        let character = rules.split(" ").nth(1).unwrap();
        let min = range_rules
            .split("-")
            .nth(0)
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let max = range_rules
            .split("-")
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let password = line.split(":").nth(1).unwrap();
        let character_count = password.matches(character).count();
        println!(
            "{} {} {} {} {}",
            character, min, max, password, character_count
        );

        if min <= character_count as i32 && character_count as i32 <= max {
            good_passwords += 1;
        }
    }

    println!("Good Passwords: {}", good_passwords);
}

pub fn two_two() {
    let lines = lines_from_file("input/2.txt");
    let mut good_passwords = 0;

    for line in &lines {
        let rules = line.split(":").nth(0).unwrap();
        let range_rules = rules.split(" ").nth(0).unwrap();
        let character = rules.split(" ").nth(1).unwrap().chars().nth(0).unwrap();
        let min = range_rules
            .split("-")
            .nth(0)
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let max = range_rules
            .split("-")
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let password = line.split(":").nth(1).unwrap().trim();
        let min_equality = password.chars().nth(min as usize - 1).unwrap() == character;
        let max_equality = password.chars().nth(max as usize - 1).unwrap() == character;
        let mut good = false;
        if (min_equality || max_equality) && !(min_equality && max_equality) {
            good_passwords += 1;
            good = true;
        }
        println!(
            "{} {} {} {} {} {} {}",
            password, character, min, max, min_equality, max_equality, good
        );
    }

    println!("Good Passwords: {}", good_passwords);
}

pub fn three() {}
