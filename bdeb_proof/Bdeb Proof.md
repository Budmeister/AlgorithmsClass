# Bdeb Proof
The goal of this report is to prove the correctness of my solution to Bdeb. 

## Source
Here is the code of interest. Note that the function, `count_positions`, calls the recursive function, `count_positions_r`. `count_positions_r` receives a reference to the variable, `count`, which will hold the solution. This way, it can be incremented every time a solution is found, rather than trying to add up a lot of different numbers. 
```Rust
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
```

## Proof
We will prove that this is correct using mathematical induction. 

### Base Case
Let $k = 1$. Then, the correct answer is $n^2$ valid arrangements. `count_positions` will call `count_positions_r`, starting at the coordinates, `(0, 0)`. `count_positions_r` will iterate through all positions because of the double loop. `is_attacking` will always be false, because `coords` will always be empty. So, `count` will be incremented $n^2$ times, which will give us the correct answer. 

### $k + 1$ Case
Assume the algorithm be true for $k$. Now, we must prove it is true for $k+1$. In the language of the program, assume `count_positions_r(n, k - 1, coord, coords, count)` gives the correct answer. Now, we must prove that `count_positions_r(n, k, coord, coords, count)` gives the correct answer.

The `coords` vector holds the list of already existing bishops. We know this is true because `count_positions_r` is called in two places, once in `count_position`, and once in itself. In both cases, `coords` is updated before the call is made, and after the recursive call, it is updated again. Thus, assuming that the function, `Coord::is_attacking` is correctly implemented, then the local variable, `is_attacking` will be correct. 

Only if the newly added bishop is not attacking any previously added bishop (`if !is_attacking`) can we consider this as a possible valid position and move on with processing. We already considered if `k == 1`. We can't increment `count` until all bishops are on the board, so if `k > 1`, then we will call `count_positions_r` with `coords` including the newly added bishop. `count_positions_r(n, k - 1, coord, coords, count)` is assumed to be correct, so the answer to the `count_positions_r(n, k - 1, coord, coords, count)` is correct. 

The value `start`, also called `coord` (I really didn't need to unpack it into `x` and `y` and then repack it into `coord`.) moves forward throughout the function call. The assumption is that when `count_positions_r` is called with a `start` value, all `start` values before that one (that is, `x' < x || (x' == x && y' < y)`) have already been checked and counted. This is comparable to the combination counting algorithm that iterates over pairs but considers `(a, b) == (b, a)`.

```Rust
for a in 0..(n-1) {
    for b in (a+1)..n {
        println!("({}, {})", a, b);
    }
}
```