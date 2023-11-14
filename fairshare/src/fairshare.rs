use crate::input::RefCoins;
use std::collections::HashSet;
use std::ops::Deref;
use std::ops::DerefMut;

pub fn solve_fs(coins: RefCoins) -> usize {
    let (left_coins, right_coins) = coins.split_at(coins.len() / 2);
    let left_combos = get_combos(left_coins);
    let right_combos = get_combos(right_coins);

    let diff = get_min_dist(&left_combos, &right_combos);
    match diff {
        Some(diff) => diff,
        None => {
            // left or right must be empty
            let diff = left_combos
                .iter().chain(right_combos.iter())
                .min();
            match diff {
                Some(diff) => *diff,
                None => panic!("Cannot solve fairshare problem on empty coin set"),
            }
        }
    }
}

fn get_combos(coins: RefCoins) -> Vec<usize> {
    let mut combos = vec![0];
    let mut seen = HashSet::new();

    for coin in coins {
        seen.clear();
        combos.extend_from_within(0..combos.len());
        let len = combos.len();
        let (plus, minus) = combos.split_at_mut(len / 2);
        for combo in plus {
            *combo += coin;
        }
        for combo in minus {
            *combo = combo.abs_diff(*coin);
        }
        combos.retain(|x| {
            if seen.contains(x) {
                false
            } else {
                seen.insert(*x);
                true
            }
        });
    }

    combos
}

fn get_min_dist(left_combos: &[usize], right_combos: &[usize]) -> Option<usize> {
    let mut together: Vec<_> = left_combos
        .iter()
        .map(|x| Either::This(*x))
        .chain(right_combos
            .iter()
            .map(|x| Either::That(*x))
        )
        .collect();
    together.sort_by(|x, y| x.cmp(y));

    let diff = together
        .iter().zip(together.iter().skip(1))
        .filter(|(a, b)| 
            matches!(a, Either::This(_)) != matches!(b, Either::This(_))
        )
        .map(|(a, b)| **b - **a)
        .min();
    diff
}

#[derive(Clone, Copy, Hash, Debug)]
enum Either<T> {
    This(T),
    That(T),
}
impl<T> Deref for Either<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::This(t) => t,
            Self::That(t) => t,
        }
    }    
}
impl<T> DerefMut for Either<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::This(t) => t,
            Self::That(t) => t,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_combos() {
        let coins = vec![27, 41, 32, 12];
        let combos = get_combos(&coins);
        assert_eq!(combos, [112, 58, 48, 30, 88, 34, 24, 6]);
    }
}
