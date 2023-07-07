use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn main() {

    // run_a(read_the_input());
    run_b(read_the_input("input.txt"));
}

fn run_b(input: Vec<String>) {
    let entry_vec: Vec<Entry> = organize_input(&input);
    let guard_map: HashMap<u16, u32> = get_guard_times(&entry_vec);

    let mut largest = (0, 0);
    let mut current_id = 0;
    for (id, _) in &guard_map {
        let value = get_minute_most_slept(*id, &entry_vec);
        if value.1 > largest.1 {
            largest = value;
            current_id = *id;
        }
    }
    println!("{}", current_id as u32 * largest.0);
}

fn run_a(input: Vec<String>) {
    let entry_vec: Vec<Entry> = organize_input(&input);
    let guard_map: HashMap<u16, u32> = get_guard_times(&entry_vec);


    let mut largest = 0;
    let mut current_id = 0;
    for (id, value) in &guard_map {
        if *value > largest {
            largest = *value;
            current_id = *id;
        }
    }

    let ans = get_minute_most_slept(current_id, &entry_vec);
    println!("{}", dbg!(&ans.0) * current_id as u32);
    dbg!(&guard_map);
}

fn get_minute_most_slept(guard_id: u16, entry_vec: &Vec<Entry>) -> (u32, u32) {
    let mut sleep_times: HashMap<u32, u32> = HashMap::new();
    let mut current_guard: u16 = 0;
    let mut sleep_start = 0;
    for entry in entry_vec {
        // dbg!(&entry);
        if entry.action.contains("Guard") {
            let pattern = r"Guard #(\d+) begins shift";
            let re = Regex::new(pattern).unwrap();
            if let Some(captures) = re.captures(&entry.action) {
                let number: u16 = captures[1].parse().unwrap();
                current_guard = number;
            }
        } else if entry.action.contains("falls asleep") && current_guard == guard_id {
            sleep_start = entry.minute;
        } else if current_guard == guard_id {
            for min in sleep_start..entry.minute {
                let entry2 = sleep_times.entry(min as u32).or_insert(0);
                *entry2 += 1;
            }
        }
    }
    let mut largest = 0;
    let mut current_id = 0;
    for (id, value) in sleep_times {
        if value > largest {
            largest = value;
            current_id = id;
        }
    }
    (current_id, largest)
}

fn get_guard_times(entry_vec: &Vec<Entry>) -> HashMap<u16, u32> {
    let mut guard_map: HashMap<u16, u32> = HashMap::new();
    let mut current_guard: u16 = 0;
    let mut sleep_start = 0;
    for entry in entry_vec {
        // dbg!(&entry);
        if entry.action.contains("Guard") {
            let pattern = r"Guard #(\d+) begins shift";
            let re = Regex::new(pattern).unwrap();
            if let Some(captures) = re.captures(&entry.action) {
                let number: u16 = captures[1].parse().unwrap();
                current_guard = number;
                guard_map.entry(current_guard).or_insert(0);
            }
        } else if entry.action.contains("falls asleep") {
            sleep_start = entry.minute;
        } else {
            let entry2 = guard_map.entry(current_guard).or_insert(0);
            *entry2 += entry.minute as u32 - sleep_start as u32;
        }
    }

    guard_map
}

fn organize_input(input: &Vec<String>) -> Vec<Entry> {
    let mut entry_vec: Vec<Entry> = input.iter()
                                         .map(|line| Entry::build(line))
                                         .collect();
    entry_vec.sort_by_key(|entry| entry.sort_key);
    entry_vec
}

#[derive(Debug)]
struct Entry {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    action: String,
    sort_key: u64,
}

impl Entry {
    fn build(entry_string: &String) -> Entry {
        let pattern = r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (.+)";
        let re = Regex::new(pattern).unwrap();

        if let Some(captures) = re.captures(entry_string) {

            let year: u16 = captures[1].parse().unwrap();
            let month: u8 = captures[2].parse().unwrap();
            let day: u8 = captures[3].parse().unwrap();
            let hour: u8 = captures[4].parse().unwrap();
            let minute: u8 = captures[5].parse().unwrap();
            let action: String = captures[6].parse().unwrap();
            let sort_key: u64 = year as u64 * 31_536_000 +
                                 month as u64  *  2_629_746 +
                                 day as u64  * 86_400 +
                                 hour as u64  * 360 +
                                 minute as u64  * 60;

            Entry { year, month, day, hour, minute, action, sort_key }
        } else { panic!("Failed to build entry") }
    }
}

fn read_the_input(path: &str) -> Vec<String>{
    match fs::read_to_string(path) {
        Ok(data) => data.lines().map(String::from).collect(),
        Err(_) => panic!()
    }
}
