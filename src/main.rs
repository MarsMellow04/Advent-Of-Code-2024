#![feature(iter_map_windows)]
use std::env;
use std::fs;
use std::path::Path;
// The levels are all increasing or all decreasing 
// the differen is either 1 to 3

fn is_valid(vector: Vec<i32>) -> bool {
    let increasing = vector.windows(2).all(|w| w[0] <= w[1] );
    let decreasing = vector.windows(2).all(|w| w[0] >= w[1] );

    if !increasing && !decreasing {
        return false;
    }
    let iterable: Vec<_> = vector.iter().map_windows(|&[a, b]| (b - a).abs() ).collect();
    if iterable.iter().all(|&item| item >=1 && item <= 3 ) {
        return true;
    }
    return false;
}   

fn main() {
    let path = Path::new("/Users/louiscutteridge/Github/advent_of_code/week_2/test.txt");
    let contents = fs::read_to_string(path);
    let mut count = 0;

    for i in contents.expect("REASON").lines() {
        let report: Vec<_> = i.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        if is_valid(report) {
            count += 1;
        }
    }
    println!("{}", count)
}
