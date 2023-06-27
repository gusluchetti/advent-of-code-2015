use std::collections::HashMap;

const INPUT_PATH: &str = "src/inputs/d07.txt";

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = "src/inputs/test_d07.txt";

    #[test]
    fn test_sample_circuit() {
        let lines: Vec<String> = std::fs::read_to_string(INPUT_TEST)
            .expect("cargo should be run from root")
            .lines()
            .map(String::from)
            .collect();

        let circuit = set_circuit(lines);
        let res = read_circuit(&circuit);

        assert_eq!(res["d"], 72);
        assert_eq!(res["e"], 507);
        assert_eq!(res["f"], 492);
        assert_eq!(res["g"], 114);
        assert_eq!(res["h"], 65412);
        assert_eq!(res["i"], 65079);
        assert_eq!(res["x"], 123);
        assert_eq!(res["y"], 456);
    }
}

fn main() {
    let lines: Vec<String> = std::fs::read_to_string(INPUT_PATH)
        .expect("cargo should be run from root")
        .lines()
        .map(String::from)
        .collect();

    task1(lines);
    // task2();
}

// runs until beginning of circuit to get u16 value
fn run_backwards(
    circuit: &HashMap<String, Vec<String>>,
    results: &mut HashMap<String, u16>,
    sender: &str,
) -> u16 {
    let msg = results.get(sender);
    if msg != None {
        let signal = msg.unwrap().clone();
        println!("known value for {sender}: {:?}", signal);
        return signal;
    }

    // sender is either &str or number
    let mut value = sender.parse::<u16>();
    println!("got {:?} from {:?}", value, sender);
    let mut last = sender;
    let mut cur = sender;
    while value.is_err() {
        last = cur;
        if circuit[last].len() == 2 {
            cur = circuit[last][1].as_str();
        } else {
            cur = circuit[last][0].as_str();
        }
        value = cur.parse::<u16>();
        println!("looked back, got {:?} from {:?}", value, cur);
    }

    let signal = value.unwrap();
    if last.parse::<u16>().is_err() {
        println!("saving {last} with {signal}");
        results.entry(last.to_string()).or_insert(signal);
    }
    return signal;
}

fn set_circuit(lines: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut circuit: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let mut split: Vec<String> = line.split_whitespace().map(|f| f.to_string()).collect();

        let receiver = split.pop().expect("should always have receiver");
        split.pop().unwrap(); // disconsider '->'
        let sender = split[..].to_vec();

        circuit.entry(receiver.to_string()).or_insert(sender);
    }

    circuit
}

fn read_circuit(circuit: &HashMap<String, Vec<String>>) -> HashMap<String, u16> {
    let mut results: HashMap<String, u16> = HashMap::new();

    for (receiver, sender) in circuit {
        println!("{:?} receives {:?}", receiver, sender);
        let mut msg: u16 = 0;

        if results.get(receiver) != None {
            msg = results.get(receiver).unwrap().clone();
            println!("receiver ({:?}) is known already as {:?}", receiver, msg);
            continue;
        }

        match sender.len() {
            3 => {
                let first = run_backwards(circuit, &mut results, sender[0].as_str());
                let second = run_backwards(circuit, &mut results, sender[2].as_str());

                msg = match sender[1].as_str() {
                    "AND" => first & second,
                    "OR" => first | second,
                    "RSHIFT" => first >> second,
                    "LSHIFT" => first << second,
                    _ => 0 as u16,
                }
            }
            // NOT
            2 => {
                msg = !run_backwards(circuit, &mut results, sender[1].as_str());
            }
            // just a value assignment
            1 => {
                msg = run_backwards(circuit, &mut results, sender[0].as_str());
            }
            _ => {}
        }
        results.entry(receiver.to_string()).or_insert(msg);
        println!("READ: saving {receiver} with {msg}");
    }

    results
}

fn task1(lines: Vec<String>) -> () {
    let circuit = set_circuit(lines);
    let res = read_circuit(&circuit);

    println!("{:?}\n", res["a"]);
}

fn task2() -> () {}
