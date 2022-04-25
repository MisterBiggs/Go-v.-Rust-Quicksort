#![feature(test)]

use rayon::join;

fn choose_pivot<T: Ord>(slice: &[T]) -> usize {
    let (mut left, mid, mut right) = (0, slice.len() / 2, slice.len() - 1);
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

fn partition<T: Ord>(slice: &mut [T], pivot: usize) -> usize {
    let mxix = slice.len() - 1;
    slice.swap(pivot, mxix);
    let (mut left, mut right) = (0, mxix - 1);

    while left < right {
        if slice[left] <= slice[mxix] {
            left += 1;
        } else if slice[right] >= slice[mxix] {
            right -= 1;
        } else {
            slice.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    if left > right {
        // We just swapped the final two.
        slice.swap(left, mxix);
        return left;
    }

    // Left and right met.
    if slice[left] >= slice[mxix] {
        slice.swap(left, mxix);
        return left;
    } else if slice[left] <= slice[mxix] {
        slice.swap(left + 1, mxix);
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
    let (left_slice, right_slice) = slice.split_at_mut(pivot);

    let right_slice = &mut right_slice[1..]; // want to exclude pivot

    quicksort(left_slice);
    quicksort(right_slice);
}

fn par_quicksort<T: Ord + std::marker::Send>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    } else if slice.len() == 2 {
        if slice[0] >= slice[1] {
            slice.swap(0, 1);
        }
        return;
    }

    let pivot = partition(slice, choose_pivot(slice));
    let (left_slice, right_slice) = slice.split_at_mut(pivot);
    let right_slice = &mut right_slice[1..]; // want to exclude pivot

    join(|| quicksort(left_slice), || quicksort(right_slice));
}

use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
fn main() {
    let mut s: Vec<i32> = Uniform::from(0..1_000_000_000)
        .sample_iter(&mut thread_rng())
        .take(100_000)
        .collect();

    quicksort(&mut s);
}

extern crate test;
use test::Bencher;
fn get_bench_vec() -> Vec<i32> {
    let s = Uniform::from(0..1_000_000_000)
        .sample_iter(&mut thread_rng())
        .take(500_000)
        .collect();
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
