use std::fs;

const INPUT_PATH: &str = "src/inputs/d02.txt";

#[derive(Debug)]
struct Size {
    length: u32,
    width: u32,
    height: u32,
}

impl Size {
    fn surface_area(&self) -> u32 {
        let two_lw = 2 * self.length * self.width;
        let two_wh = 2 * self.width * self.height;
        let two_hl = 2 * self.height * self.length;

        two_lw + two_wh + two_hl
    }

    fn smallest_side(&self) -> u32 {
        let mut vec = vec![self.length, self.width, self.height];
        vec.sort();
        vec[0] * vec[1]
    }

    fn ribbon_size(&self) -> u32 {
        let mut vec = vec![self.length, self.width, self.height];
        vec.sort();
        2 * vec[0] + 2 * vec[1]
    }

    fn cubic_volume(&self) -> u32 {
        self.length * self.width * self.height
    }
}

fn main() {
    let file: String = fs::read_to_string(INPUT_PATH).expect("should have input");
    task1(&file);
    task2(&file);
}

fn task1(file: &String) -> () {
    let input: Vec<&str> = file.lines().collect();
    let mut total_paper = 0;

    for n in input {
        let split: Vec<&str> = n.split_terminator('x').collect();
        if split.len() == 3 {
            let length: u32 = split[0].parse().unwrap();
            let width: u32 = split[1].parse().unwrap();
            let height: u32 = split[2].parse().unwrap();

            let box_size = Size {
                length,
                width,
                height,
            };
            let box_paper = box_size.surface_area() + box_size.smallest_side();
            total_paper = total_paper + box_paper;
        }
    }
    println!("TOTAL paper needed: {}", total_paper);
}

fn task2(file: &String) -> () {
    let input: Vec<&str> = file.lines().collect();
    let mut total_ribbon = 0;

    for n in input {
        let split: Vec<&str> = n.split_terminator('x').collect();
        if split.len() == 3 {
            let length: u32 = split[0].parse().unwrap();
            let width: u32 = split[1].parse().unwrap();
            let height: u32 = split[2].parse().unwrap();

            let box_size = Size {
                length,
                width,
                height,
            };
            let ribbon = box_size.ribbon_size() + box_size.cubic_volume();
            total_ribbon = total_ribbon + ribbon;
        }
    }
    println!("TOTAL ribbon length needed: {}", total_ribbon);
}
