use std::fs;

use md5;
use md5::Digest;

const INPUT_PATH: &str = "src/inputs/d04.txt";

#[cfg(test)]
mod tests {
    use super::*;
    use md5;
    use md5::Digest;
    #[test]
    fn validate_md5_hash() {
        let secret = "abcdef";
        let answer = 609043;
        let source = format!["{secret}{answer}"];

        let hash: Digest = md5::compute(source);
        let hash = format!("{:x}", hash);
        assert!(hash.contains("00000"));
    }

    #[test]
    fn test_hash_find() {
        let secret = "pqrstuv";
        let answer = 1048970;
        let lowest = find_lowest_number(secret, 5);

        assert_eq!(lowest, answer);
    }
}

pub fn find_lowest_number(secret: &str, num_zeroes: u8) -> u32 {
    let mut lowest = 0;
    let mut zeroes: String = "".to_string();
    for _ in 0..num_zeroes {
        zeroes.push('0')
    }

    for n in 1.. {
        let source = format!["{secret}{n}"];
        let hash: Digest = md5::compute(source);
        let hash = format!("{:x}", hash);
        println!("at {n} - {hash}");
        if hash.starts_with(zeroes.as_str()) {
            lowest = n;
            break;
        }
    }
    lowest
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).expect("should have file");
    task1(&input);
    task2(&input);
}

fn task1(input: &String) {
    let secret: &str = input.trim();
    let lowest = find_lowest_number(secret, 5);
    println!("lowest number w/ 5 leading zeros is: {lowest}");
}

fn task2(input: &String) {
    let secret: &str = input.trim();
    let lowest = find_lowest_number(secret, 6);
    println!("lowest number w/ 6 leading zeros is: {lowest}");
}
