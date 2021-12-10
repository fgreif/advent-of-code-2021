
fn main() {


    let res1 = process(include_str!("input.txt"));

    //part 2
    let res = process_part2(include_str!("input.txt"));


    //part 1
    for i in res1 {
        println!("{}", i);
    }
    let most_c = most_common(res1);
    for i in most_c {
        print!("{}", i);
    }
}

// 010100010100
// 101011101011

fn most_common(input: [usize; 12]) -> [usize; 12]{
    let mut ret: [usize; 12] = [0; 12];
    let mut counter = 0;
    for i in input {
        if i > 500 {
            ret[counter] = 1;
        }
        counter += 1;
    } return ret;
}
// 0 1 0 1 0    20
// 1 0 1 0 1   21

fn process_part2(input: &str) {
    let mut as_vec = read_part2(input);
    let mut as_vec_co2 = read_part2(input);

    let mut oxygen = iterate_part2_oxygen(as_vec);
    let mut co2 = iterate_part2_co2(as_vec_co2);
    println!("{}", co2.len());


    for i in oxygen {
        println!("{}", i);
    }
    for i in co2 {
        println!("{}", i);
    }

}
fn iterate_part2_co2(processed: Vec<&str>) -> Vec<&str> {
    let mut proc = processed;
    let mut update_proc: Vec<&str> = Vec::new();
    for x in 0..12 {
        update_proc = co2(x, proc.to_vec());
        proc = update_proc;
        if proc.len() == 1 {
            return proc;
        }

    }
    return proc;
}

fn iterate_part2_oxygen(processed: Vec<&str>) -> Vec<&str> {
    let mut proc = processed;
    let mut update_proc: Vec<&str> = Vec::new();
    for x in 0..12 {
        update_proc = oxygen(x, proc.to_vec());
        proc = update_proc;
    }
    return proc;
}


fn co2(pos: usize, input: Vec<&str>) -> Vec<&str> {
    let mut ones_str: Vec<&str> = Vec::new();
    let mut zeroes_str: Vec<&str> = Vec::new();

    for i in input {
        if i.chars().nth(pos) == Some('1') {
            ones_str.push(i);
        } else {
            zeroes_str.push(i);
        }
    }
    //most common:
    if zeroes_str.len() <= ones_str.len() {
        return zeroes_str;
    } else {
        return ones_str;
    }
}

fn oxygen(pos: usize, input: Vec<&str>) -> Vec<&str> {
    let mut ones_str: Vec<&str> = Vec::new();
    let mut zeroes_str: Vec<&str> = Vec::new();

    for i in input {
        if i.chars().nth(pos) == Some('1') {
            ones_str.push(i);
        } else {
            zeroes_str.push(i);
        }
    }
    //most common:
    if ones_str.len() >= zeroes_str.len() {
        return ones_str;
    } else {
        return zeroes_str;
    }
}

fn read_part2(input: &str) -> Vec<&str>{
    let mut amount_vec: Vec<&str>= Vec::new();
    for line in input.trim().split('\n') {
        amount_vec.push(line);
    }
    return amount_vec;

}


fn process(input: &str) -> [usize; 12]  {
    let mut amount_vec = [0; 12];
    for line in input.trim().split('\n') {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                amount_vec[i] += 1;              
            }      
        }
    } return amount_vec;
}
