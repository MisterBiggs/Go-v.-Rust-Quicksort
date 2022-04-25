#![feature(test)]

use rayon::join;

fn choose_pivot<T: Ord>(slice: &[T]) -> usize {
    // if slice.len() <= 2 {return slice.len() - 1;};
    let (mut ismall, imid, mut ibig) = (0, slice.len() / 2, slice.len() - 1);
    if slice[ibig] < slice[ismall] {
        std::mem::swap(&mut ibig, &mut ismall);
    }
    if slice[imid] <= slice[ismall] {
        ismall
    } else if slice[ibig] <= slice[imid] {
        ibig
    } else {
        imid
    }
}

/// choose a pivot, then reorder so that everything to the left of the pivot is smaller, and
/// everything to the right is greater
/// Assumes slice.len() > 2
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

    panic!(
        "This should be unreachable. Indices: {}, {} / {}",
        left, right, mxix
    );
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

    let pivot = choose_pivot(slice);
    let pivot = partition(slice, pivot);
    let (left_slice, right_slice) = slice.split_at_mut(pivot);

    let right_slice = &mut right_slice[1..];

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

    let pivot = choose_pivot(slice);
    let pivot = partition(slice, pivot);
    let (left_slice, right_slice) = slice.split_at_mut(pivot);
    // left_slice is [0 - pivot-1], right_slice is [pivot, end]. We don't want to include the
    // pivot, so reassign right_slice
    let right_slice = &mut right_slice[1..];

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
