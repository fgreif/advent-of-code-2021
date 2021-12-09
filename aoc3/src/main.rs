fn main() {
    let res = process(include_str!("input.txt"));
    for i in res {
        println!("{}", i);
    }
}

fn process(input: &str) -> [usize; 12]  {
    let mut amount_vec = [0; 12];

    for line in input.trim().split('\n') {
        let mut i = 0;
        for c in line.chars() {
            if c == '1' {
                amount_vec[i] += 1;
                i += 1;
            }
        }
    } return amount_vec;
}
