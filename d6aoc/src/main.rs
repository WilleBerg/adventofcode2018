use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

const ARRAY_WIDTH: usize = 356;
const ARRAY_HEIGHT: usize = 360;
// const ARRAY_WIDTH: usize = 10;
// const ARRAY_HEIGHT: usize = 10;

fn main() {
    let input = read_input("input.txt");
    println!("Here");
    run_a(&input);
}

fn run_a(input: &Vec<String>) {
    let mut id = 0;
    let point_list: Vec<Point> = input.iter()
        .map(|line| { id += 1; Point::build(line, id) })
        .collect();
    println!("Here1");

    let mut grid: Box<[[(u32, u32); ARRAY_WIDTH]; ARRAY_HEIGHT]> = Box::new([[(0, 0); ARRAY_WIDTH]; ARRAY_HEIGHT]);
    println!("Here2");

    for point in &point_list {
        grid[point.y as usize][point.x as usize] = (point.unique_id, 0);
        expand_grid(&mut grid, &point);
    }
    println!("Here3");

    let mut set: HashSet<u32> = HashSet::new();
    let mut x = 0;
    let mut y = 0;

    for row in grid.iter() {
        for col in row.iter() {
            if y == 0 || x == 0 || x == ARRAY_WIDTH - 1 || y == ARRAY_HEIGHT - 1 {
                set.insert(col.0);
            }
            x += 1;
        }
        y += 1;
    }

    let mut map: HashMap<u32, i32> = HashMap::new();
    let mut largest = 0;
    let mut valu = 0;
    for row in grid.iter() {
        for (val, _) in row.iter() {
            print!("{}, ", val);
            if set.contains(val) {
                continue;
            }
            if *val == 0 {
                continue;
            }
            let entry = map.entry(*val).or_insert(0);
            *entry += 1;
            if *entry > largest {
                largest = *entry;
                valu = *val;
            }
        }
        println!();
    }

    println!("{}, {}", largest, valu);
}

fn expand_grid(grid: &mut Box<[[(u32, u32); ARRAY_WIDTH]; ARRAY_HEIGHT]>, point: &Point) {
    let mut curr_x: i32 = 0;
    let mut curr_y: i32 = 0;
    for row in grid.iter_mut() {
        for (unique_id, range) in row.iter_mut() {
            if curr_x == point.x && curr_y == point.y {
                // println!("{},{}", point.x, point.y);
                curr_x += 1;
                continue;
            }
            let distance: i32 = (point.x - curr_x).abs() + (point.y - curr_y).abs();
            let dist_u32 = distance as u32;
            if (dist_u32 < *range)
            || (*unique_id, *range) == (0, 0) {
                *range = dist_u32;
                *unique_id = point.unique_id;
            } else if dist_u32 == *range {
                *unique_id = 0;
            }
            curr_x += 1;
        }
        curr_x = 0;
        curr_y += 1;
    }
}

struct Point {
    x: i32,
    y: i32,
    unique_id: u32,
}

impl Point {
    fn build(line: &String, id: u32) -> Point {
        let list = line.split(", ")
            .map(|val| val.parse().unwrap())
            .collect::<Vec<i32>>();
        Point { x: list[0], y: list[1], unique_id: id }
    }
}

fn read_input(path: &str) -> Vec<String> {
    fs::read_to_string(path).unwrap()
        .lines()
        .map(|s| String::from(s))
        .collect()
}
