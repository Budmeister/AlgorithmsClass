use std::{io, ops::Add, env, time::SystemTime};

pub trait AbsDiff {
    fn abs_diff(&self, other: Self) -> Self;
}
impl AbsDiff for u32 {
    fn abs_diff(&self, other: Self) -> Self {
        u32::abs_diff(*self, other)
    }
}

pub trait Distance<T, X> {
    fn dist(&self, other: &T) -> X;
}

pub trait Point<X> {
    fn get_x(&self) -> X;
    fn get_y(&self) -> X;
}
impl<X, T> Distance<T, X> for T
where
    X: Ord + AbsDiff + Add<X, Output = X>,
    T: Point<X>,
{
    fn dist(&self, other: &T) -> X {
        self.get_x().abs_diff(other.get_x()) + self.get_y().abs_diff(other.get_y())
    }
}

fn indirect_sort_by<T, F>(slice: &[T], compare: F) -> Vec<&T>
where
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let mut indices: Vec<&T> = slice.iter().collect();
    indices.sort_by(|a, b| compare(a, b));
    indices
}

fn _closest_pair_brute_force<'a, X, T>(p: &[&'a T]) -> (&'a T, &'a T, X)
where
    X: Ord + AbsDiff + Add<X, Output = X> + Copy,
    T: Point<X>,
{
    let mut next = p;

    let mut min: Option<(&'a T, &'a T, X)> = None;
    while let Some((p1, rest)) = next.split_first() {
        for p2 in rest {
            let dist = p1.dist(p2);
            match min {
                Some((_, _, delta)) => {
                    if dist < delta {
                        min = Some((p1, p2, dist));
                    }
                }
                None => {
                    min = Some((p1, p2, dist));
                }
            }
        }
        next = rest;
    }
    min.expect("Cannot use empty array")
}

fn _closest_pair_r<'a, X, T>(x: &[&'a T], y: &[&'a T]) -> (&'a T, &'a T, X)
where
    X: Ord + AbsDiff + Add<X, Output = X> + Copy,
    T: Point<X>,
{
    if x.len() <= 3 {
        return _closest_pair_brute_force(x);
    }

    // Divide x into xl and xr based on l_x
    let (xl, xr) = x.split_at(x.len() / 2);
    
    // Divide y into yl and yr based on l_x
    let l_x = x[x.len() / 2].get_x();
    let mut yl: Vec<&'a T> = Vec::new();
    let mut yr: Vec<&'a T> = Vec::new();
    for point in y {
        if point.get_x() <= l_x {
            yl.push(point);
        } else {
            yr.push(point);
        }
    }

    // Conquer
    let l = _closest_pair_r(xl, &yl);
    let r = _closest_pair_r(xr, &yr);

    let mut min = if l.2 < r.2 { l } else { r };

    for i in 0..y.len() {
        let end = y.len().min(i + 10);
        let points = &y[i..end];

        let (p1, points) = points.split_first().expect("Found zero points");
        for p2 in points {
            let dist = p1.dist(p2);
            if dist < min.2 {
                min = (p1, p2, dist);
            }
        }
    }

    min
}

pub fn closest_pair<X, T>(q: &[T]) -> (&T, &T, X)
where
    X: Ord + AbsDiff + Add<X, Output = X> + Copy,
    T: Point<X>,
{
    let x = indirect_sort_by(q, |a, b| a.get_x().cmp(&b.get_x()));
    let y = indirect_sort_by(q, |a, b| a.get_y().cmp(&b.get_y()));
    _closest_pair_r(&x, &y)
}

pub fn closest_pair_brute_force<X, T>(q: &[T]) -> (&T, &T, X)
where
    X: Ord + AbsDiff + Add<X, Output = X> + Copy,
    T: Point<X>,
{
    _closest_pair_brute_force(&q.iter().collect::<Vec<_>>())
}

struct Animal {
    taz: u32,
    tav: u32,
    name: String,
}
impl Point<u32> for Animal {
    fn get_x(&self) -> u32 {
        self.taz
    }
    fn get_y(&self) -> u32 {
        self.tav
    }
}

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read a line from stdin");
    let n: u32 = line.trim().parse().unwrap_or_else(|_| panic!("`{}` is not an integer", line));
    line.clear();

    let mut animals = Vec::new();
    for _ in 0..n {
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read a line from stdin");
        let words = line.trim().split(' ').collect::<Vec<_>>();
        if words.len() < 2 {
            panic!("Less than 2 numbers on a line: {}", line);
        }
        let ([taz, tav], words) = words[..].split_at(2) else { panic!("How?") };
        let taz: u32 = taz.parse().unwrap_or_else(|_| panic!("`{}` is not an integer", line));
        let tav: u32 = tav.parse().unwrap_or_else(|_| panic!("`{}` is not an integer", line));
        let name: String = words.join(" ");
        let animal = Animal { taz, tav, name };
        animals.push(animal);
        line.clear();
    }

    let (a1, a2, dist);
    let start = SystemTime::now();
    if env::args().any(|x| x == "--brute") {
        (a1, a2, dist) = closest_pair_brute_force(&animals);
    } else {
        (a1, a2, dist) = closest_pair(&animals);
    }
    match start.elapsed() {
        Ok(elapsed) => {
            println!("Finished in {}ms", elapsed.as_millis());
        }
        Err(e) => {
            println!("An error occurred while measureing the time: {e:?}");
        }
    }

    println!("{} and {} are most similar at {} hour-minutes", a1.name, a2.name, dist);
}
