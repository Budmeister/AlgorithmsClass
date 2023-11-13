use rand::{
    Rng,
    rngs::ThreadRng
};

fn partition<T>(arr: &mut [T], mut pivot: usize) -> usize 
    where T: Ord
{
    arr.swap(arr.len() - 1, pivot);
    pivot = arr.len() - 1;

    // x < i -> x < pivot
    let mut i = 0;
    // x >= j -> x >= pivot
    let mut j = pivot;

    while i < j {
        if arr[i] < arr[pivot] {
            i += 1;
        } else {
            j -= 1;
            arr.swap(i, j);
        }
    }
    arr.swap(pivot, j);
    j
}

fn randomized_select<T>(arr: &mut [T], i: usize, rng: &mut ThreadRng) -> Option<usize>
    where T: Ord
{
    let len = arr.len();
    match len {
        0 => None,
        1 => Some(0),
        _ => {
            let pivot: usize = rng.gen_range(0..arr.len());
            let q = partition(arr, pivot);
            use std::cmp::Ordering as O;
            match i.cmp(&q) {
                O::Equal => {
                    Some(q)
                }
                O::Less => {
                    randomized_select(&mut arr[0..q], i, rng)
                }
                O::Greater => {
                    randomized_select(&mut arr[q + 1..len], i - (q + 1), rng).map(|q_prime| q_prime + (q + 1))
                }
            }
        }
    }

}

pub fn order_statistic_rp<T>(arr: &[T], i: usize) -> Option<T>
    where T: Ord + Clone
{
    let mut arr = arr.to_vec();
    let mut rng = rand::thread_rng();

    match randomized_select(&mut arr, i, &mut rng) {
        Some(q) => {
            let t = arr.swap_remove(q);
            Some(t)
        }
        None => None,
    }
}

fn arg_insertion_sort<'a, T>(arr: &[T], indices: &'a mut [usize])
    where T: Ord
{
    // let mut indices: Vec<_> = (0..arr.len()).into_iter().collect();
    indices
        .iter_mut()
        .enumerate()
        .for_each(|(i, x)| *x = i);

    for i in 1..arr.len() {
        for j in (0..i).rev() {
            if arr[indices[j]] > arr[indices[j + 1]] {
                indices.swap(j, j + 1)
            } else {
                break;
            }
        }
    }
}

fn median_of_five<T>(arr: &[T], storage: &mut [T]) -> Option<usize>
    where T: Ord + Clone
{
    match arr.len() {
        0 => None,
        1..=5 => {
            let mut storage = [0, 0, 0, 0, 0];
            let storage_indices = &mut storage[0..arr.len()];
            arg_insertion_sort(arr, storage_indices);
            let median = storage_indices[storage_indices.len() / 2];
            Some(median)
        }
        6.. => {
            let (ours, theirs) = storage.split_at_mut(arr.len() / 5 + 1);
            for (i, start) in (0..arr.len()).step_by(5).enumerate() {
                let end = usize::min(start + 5, arr.len());
                let median = median_of_five(&arr[start..end], theirs).unwrap();
                ours[i] = arr[median].clone();
            }
            median_of_five(ours, theirs)
        }
        _ => None, // what?
    }
}

fn mof_select<T>(arr: &mut [T], i: usize, storage: &mut [T]) -> Option<usize>
    where T: Ord + Clone
{
    let len = arr.len();
    match len {
        0 => None,
        1 => Some(0),
        _ => {
            let pivot = median_of_five(arr, storage).unwrap();
            let q = partition(arr, pivot);
            use std::cmp::Ordering as O;
            match i.cmp(&q) {
                O::Equal => {
                    Some(q)
                }
                O::Less => {
                    mof_select(&mut arr[0..q], i, storage)
                }
                O::Greater => {
                    mof_select(&mut arr[q + 1..len], i - (q + 1), storage).map(|q_prime| q_prime + (q + 1))
                }
            }
        }
    }

}

pub fn order_statistic_mof<T>(arr: &[T], i: usize) -> Option<T>
    where T: Ord + Clone + Default,
{
    if arr.is_empty() {
        return None;
    }
    let mut arr = arr.to_vec();
    let mut storage = vec![T::default(); arr.len()];

    match mof_select(&mut arr, i, &mut storage) {
        Some(q) => {
            let t = arr.swap_remove(q);
            Some(t)
        }
        None => None,
    }
}
