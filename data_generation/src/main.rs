use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::thread_rng;

use std::fs::File;
use std::io::Write;

fn main() {
    {
        let mut nums = (0..(50e3 as usize)).collect::<Vec<usize>>();
        nums.shuffle(&mut thread_rng());

        let num_str = nums
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        let mut fp = File::create("../data/perm50e3.txt").expect("unable to create file");
        fp.write_all(num_str.as_bytes()).expect("failed to write");
    }

    {
        let mut nums = (0..(50e6 as usize)).collect::<Vec<usize>>();
        nums.shuffle(&mut thread_rng());

        let num_str = nums
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        let mut fp = File::create("../data/perm50e6.txt").expect("unable to create file");
        fp.write_all(num_str.as_bytes()).expect("failed to write");
    }

    {
        let nums: Vec<i32> = Uniform::from(0..1_000_000_000)
            .sample_iter(&mut thread_rng())
            .take(50e6 as usize)
            .collect();

        let num_str = nums
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        let mut fp = File::create("../data/billion50e6.txt").expect("unable to create file");
        fp.write_all(num_str.as_bytes()).expect("failed to write");
    }
    {
        let nums: Vec<f32> = Uniform::from(0.0..10.0)
            .sample_iter(&mut thread_rng())
            .take(50e3 as usize)
            .collect();

        let num_str = nums
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        let mut fp = File::create("../data/floats50e3.txt").expect("unable to create file");
        fp.write_all(num_str.as_bytes()).expect("failed to write");
    }
}
