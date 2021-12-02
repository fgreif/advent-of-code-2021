use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let result_p1 = read_file();
    let result_p2 = result_p1.clone();
    
    //problem1:
    let mut sum = 1;
    for current in 0..result_p1.len()-1 {
        if result_p1[current] < result_p1[current+1] {
            sum += 1;
        }
    }
    println!("Problem 1: {}", sum);

    //problem2:
    let res = calc_nums(result_p2);
    println!("Problem 2: {}", res);

}

fn read_file() -> Vec<String> {
    let reader = BufReader::new(File::open("input1.txt").expect("Cannot open input1.txt"));
    let mut words = Vec::new();

    for line in reader.lines() {
        words.push(line.unwrap());
    }
    return words;
}

fn calc_nums(words: Vec<String>) -> usize {
    let sums = sum_sliding_window(words);
    sums.windows(2)
        .filter(|pair| pair[0] < pair[1])
        .count()
}

fn sum_sliding_window(words: Vec<String>) -> Vec<i32> {
    let nums = convert_to_num(words);
    let mut sum_list = Vec::new();
    for current_num in 1..nums.len()-1 {
        let a = nums[current_num-1];
        let b = nums[current_num];
        let c = nums[current_num+1];
        let sum = get_sum(a, b, c);
        sum_list.push(sum);
    }
    return sum_list;
}

fn convert_to_num(words: Vec<String>) -> Vec<i32> {
    let mut nums = Vec::new();
    for word in words {
        nums.push(word.parse().unwrap());
    }
    return nums;
}

fn get_sum(num1: i32, num2: i32, num3:i32) -> i32 {
    num1 + num2 + num3
}
