mod order_statistic;
use order_statistic::{
    order_statistic_rp,
    order_statistic_mof,
};

fn main() {
    let arr = [90, 30, 20, 66, 90, 33, 73, 30, 14, 89, 29, 24, 70, 51, 84, 95,  9,
                            50,  5, 79, 77, 40, 23, 30,  1, 42, 56, 77, 27,  6, 98, 33, 77, 64,
                            29, 19, 60, 30, 59, 45,  2, 92, 97, 82, 44, 84, 13, 73, 67, 84, 81,
                            95, 10, 92, 96, 74, 15, 36,  0, 25, 79, 68, 31, 97, 86, 26, 69, 63,
                            88, 40, 27, 77, 46, 98,  1, 84, 22, 86, 39, 21, 69, 32,  5, 93, 84,
                            93, 39, 63, 28, 54, 91, 84,  6, 64, 58, 86, 43, 60, 50,  4];
    println!("Original: {:?}", arr);

    let sorted: Vec<_> = (0..arr.len())
        .into_iter()
        .map(|i| order_statistic_rp(&arr, i).unwrap())
        .collect();
    println!("Sorted: {:?}", sorted);

    let sorted: Vec<_> = (0..arr.len())
        .into_iter()
        .map(|i| order_statistic_mof(&arr, i).unwrap())
        .collect();
    println!("Sorted: {:?}", sorted);
}
