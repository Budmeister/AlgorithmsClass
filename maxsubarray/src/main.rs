use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("IOError while reading line");
    let line = strip_trailing_newline(&line);
    let n = line.parse().expect(&format!("Invalid number: \"{}\"", line));
    for _ in 0..n {
        let nums = get_input();
        let (start, end, sum) = find_max_subarray(&nums);
    
        println!("{} {} {}", start, end, sum);
    }
}

fn get_input() -> Box<[i32]> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("IOError while reading line");
    let n = strip_trailing_newline(&mut line);
    let _n = n.parse::<i32>().expect(&format!("Invalid number: \"{}\"", n));

    line.clear();
    io::stdin().read_line(&mut line).expect("IOError while reading line");
    
    let mut nums = Vec::new();
    let line = strip_trailing_newline(&line);
    for num in line.split(" ") {
        if num.is_empty() {
            continue;
        }
        let num = num.parse().expect(&format!("Invalid number: \"{}\"", num));
        nums.push(num);
    }
    nums.into_boxed_slice()
}

fn find_max_subarray(nums: &[i32]) -> (usize, usize, i32) {
    let mut start = 0;
    let mut end = 0;
    let mut sum = 0;    // An array of all negatives should have a max subarray of (0, 0, 0)
    let mut temp_start = 0;
    let mut temp_end = 0;
    let mut temp_sum = 0;

    for (i, num) in nums.iter().enumerate() {
        temp_end += 1;
        temp_sum += num;

        if temp_sum > sum {
            start = temp_start;
            end = temp_end;
            sum = temp_sum;
        } else if temp_sum < 0 {
            temp_start = temp_end;
            temp_sum = 0;
        }
    }
    
    (start, end, sum)
}

// https://stackoverflow.com/questions/37888042/remove-single-trailing-newline-from-string-without-cloning
fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}
