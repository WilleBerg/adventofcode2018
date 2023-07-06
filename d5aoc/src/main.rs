use std::fs;

fn main() {
    let mut input: Vec<char> = fs::read_to_string("input.txt")
        .unwrap().chars().collect::<Vec<char>>();
    // run_a(&mut input);
    run_b(&mut input);
}

fn run_a(input: &mut Vec<char>) {
    let mut counter = 0;
    loop {
        if counter == input.len() - 1 {
            break;
        }
        let pred1 = input[counter].to_ascii_lowercase() == input[counter + 1];
        let pred2 = input[counter].to_ascii_uppercase() == input[counter + 1];
        let pred3 = input[counter] != input[counter + 1];
        if (pred1 || pred2) && pred3 {
            input.remove(counter);
            input.remove(counter);
            if counter != 0 {
                counter -= 1;
            }
        } else {
            counter += 1;
        }
    }
    let out = String::from(input.iter().collect::<String>().trim());
    println!("{}, {}", out, out.len());
}

fn run_b(input: &mut Vec<char>) {
    let mut shortest = core::usize::MAX;
    let alph = String::from("abcdefghijklmnopqrstuvwxyz").chars().collect::<Vec<_>>();
    for c in alph {
        let test_inp = input.iter()
                                       .filter(|&ac| ac.to_ascii_lowercase() != c)
                                       .cloned()
                                       .collect();
        let result = get_poly(test_inp);
        if result < shortest {
            shortest = result;
        }
    }
    println!("{shortest}");
}

fn get_poly(mut input: Vec<char>) -> usize {
    let mut counter = 0;
    loop {
        if counter == input.len() - 1 {
            break;
        }
        let pred1 = input[counter].to_ascii_lowercase() == input[counter + 1];
        let pred2 = input[counter].to_ascii_uppercase() == input[counter + 1];
        let pred3 = input[counter] != input[counter + 1];
        if (pred1 || pred2) && pred3{
            input.remove(counter);
            input.remove(counter);
            if counter != 0 {
                counter -= 1;
            }
        } else {
            counter += 1;
        }
    }
    let out = String::from(input.iter().collect::<String>().trim());
    println!("{out}");
    out.len()
}
