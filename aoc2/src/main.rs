use std::{io::{BufReader, BufRead}, fs::File};

fn main() {
    println!("{}", part1(include_str!("input.txt")));
    println!("{}", part2(include_str!("input.txt")));
}

fn part1(input: &str) -> i64{
    let mut x = 0;
    let mut y = 0;

    for line in input.trim().split('\n') {
        let mut parts = line.split(' ');
        let direction = parts.next().unwrap();
        let amount: i64 = parts.next().unwrap().parse().unwrap();
        match direction {
            "forward" => {
                x += amount;
            }
            "down" => {
                y -= amount;
            }
            "up" => {
                y += amount;
            }
            _ => panic!(),
        }
    }
    x * y
}


fn part2(input: &str) -> i64{
    let mut x = 0;
    let mut y = 0;
    let mut aim: i64 = 0;


    for line in input.trim().split('\n') {
        let mut parts = line.split(' ');
        let direction = parts.next().unwrap();
        let amount: i64 = parts.next().unwrap().parse().unwrap();
        match direction {
            "forward" => {
                x += amount;
                y -= aim * amount;
            }
            "down" => {   
                aim -= amount;
            }
            "up" => {
                aim += amount;
            }
            _ => panic!(),
        }
    }
    x * y
}

