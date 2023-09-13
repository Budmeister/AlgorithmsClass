use std::io;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

fn main() {
    let mut line = String::new();
    loop {
        line.clear();
        let (n, k) = get_input(&mut line);
        if n == 0 && k == 0 {
            break;
        }

        let count = count_positions(n, k);

        println!("{}", count);
    }
}

fn get_input(line: &mut String) -> (usize, usize) {
    io::stdin().read_line(line).expect("IOError while reading line");
    let nums: Vec<_> = line.split(" ").collect();
    if let [n, k] = nums[..] {
        let n = strip_trailing_newline(n);
        let k = strip_trailing_newline(k);
        // println!("n: \"{}\", k: \"{}\"", n, k);
        let n = n.parse().expect(&format!("Invalid number: \"{}\"", n));
        let k = k.parse().expect(&format!("Invalid number: \"{}\"", k));
        if n > isize::MAX as usize {
            panic!("Input n too big: {}", n);
        }
        if k > isize::MAX as usize {
            panic!("Input k too big: {}", k);
        }
        (n, k)
    } else {
        panic!("Invalid number of numbers on line: \"{}\"", line);
    }
}

// https://stackoverflow.com/questions/37888042/remove-single-trailing-newline-from-string-without-cloning
fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

fn count_positions(n: usize, k: usize) -> usize {
    let mut coords: Vec<Coord> = Vec::new();
    let mut count = 0;
    let start = Coord { x: 0, y: 0 };

    count_positions_r(n, k, start, &mut coords, &mut count);

    count
}

fn count_positions_r(n: usize, k: usize, start: Coord, coords: &mut Vec<Coord>, count: &mut usize) {
    // println!("n: {}, k: {}", n, k);
    // Only look at positions beyond `start`
    let Coord { mut x, mut y } = start;
    loop {
        loop {
            // println!("\tx: {}, y: {}", x, y);
            let coord = Coord { x, y };
            let is_attacking = coords
                .iter()
                .any(|already| coord.is_attacking(already));
            if !is_attacking {
                // print!("\tSafe");
                if k == 1 {
                    // println!(" - Incrementing count");
                    *count += 1;
                } else {
                    // println!();
                    coords.push(coord);
                    count_positions_r(n, k - 1, coord, coords, count);
                    coords.pop();
                }
            }
            y += 1;
            if y >= n {
                y = 0;
                break;
            }
        }
        x += 1;
        if x >= n {
            break;
        }
    }
}

impl Coord {
    fn is_attacking(&self, other: &Self) -> bool {
        self.diag1() == other.diag1() || self.diag2() == other.diag2()
    }

    fn diag1(&self) -> isize {
        self.x as isize + self.y as isize
    }

    fn diag2(&self) -> isize {
        self.x as isize - self.y as isize
    }
}
