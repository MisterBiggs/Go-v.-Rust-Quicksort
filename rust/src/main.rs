#![feature(test)]

use rayon::join;

fn choose_pivot<T: Ord>(slice: &[T]) -> usize {
    let (mut left, mid, mut right) =
        (0, slice.len() / 2, slice.len() - 1);
    if slice[right] < slice[left] {
        std::mem::swap(&mut right, &mut left);
    }
    if slice[mid] <= slice[left] {
        return left;
    } else if slice[right] <= slice[mid] {
        return right;
    } else {
        return mid;
    }
}

fn partition<T: Ord>(
    slice: &mut [T],
    pivot: usize,
) -> usize {
    let mid = slice.len() - 1;
    slice.swap(pivot, mid);
    let (mut left, mut right) = (0, mid - 1);

    while left < right {
        if slice[left] <= slice[mid] {
            left += 1;
        } else if slice[right] >= slice[mid] {
            right -= 1;
        } else {
            slice.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    if left > right {
        // We just swapped the final two.
        slice.swap(left, mid);
        return left;
    }

    // Left and right met.
    if slice[left] >= slice[mid] {
        slice.swap(left, mid);
        return left;
    } else if slice[left] <= slice[mid] {
        slice.swap(left + 1, mid);
        return left + 1;
    }

    panic!("partition failed.")
}

fn quicksort<T: Ord + std::marker::Send>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    } else if slice.len() == 2 {
        if slice[0] >= slice[1] {
            slice.swap(0, 1);
        }
        return;
    }

    let pivot = partition(slice, choose_pivot(slice));
    let (left_slice, right_slice) =
        slice.split_at_mut(pivot);

    let right_slice = &mut right_slice[1..]; // want to exclude pivot

    quicksort(left_slice);
    quicksort(right_slice);
}

fn par_quicksort<T: Ord + std::marker::Send>(
    slice: &mut [T],
) {
    if slice.len() <= 1 {
        return;
    } else if slice.len() == 2 {
        if slice[0] >= slice[1] {
            slice.swap(0, 1);
        }
        return;
    }

    let pivot = partition(slice, choose_pivot(slice));
    let (left_slice, right_slice) =
        slice.split_at_mut(pivot);
    let right_slice = &mut right_slice[1..]; // want to exclude pivot

    join(
        || quicksort(left_slice),
        || quicksort(right_slice),
    );
}

use rand::prelude::SliceRandom;
use rand::thread_rng;
fn main() {
    let mut s = get_bench_vec();

    quicksort(&mut s);
    par_quicksort(&mut s);
}

extern crate test;
use test::Bencher;
fn get_bench_vec() -> Vec<i32> {
    let mut s = (0..20000000).collect::<Vec<i32>>();
    s.shuffle(&mut thread_rng());
    return s;
}

#[bench]
fn par_qs(b: &mut Bencher) {
    let mut test_vec = get_bench_vec();

    b.iter(|| par_quicksort(&mut test_vec))
}

#[bench]
fn qs(b: &mut Bencher) {
    let mut test_vec = get_bench_vec();

    b.iter(|| quicksort(&mut test_vec))
}

#[test]
fn qs_comp() {
    let bv = get_bench_vec();
    let mut par = bv.clone();
    let mut basic = bv.clone();

    par_quicksort(&mut par);
    quicksort(&mut basic);

    for (l, r) in par.iter().zip(basic.iter()) {
        assert_eq!(l, r)
    }
}
