/**
 * Suppose we have a one-dimensional array of random numbers between 0 and 100
 */

pub(crate) fn traverse_before_sort(mut nums: Vec<i64>) {
    // traverse
    let len = nums.len();
    for i in 0..len {
        if nums[i] < 50 {
            nums[i] = 0;
        }
    }

    nums.sort();
}


pub(crate) fn traverse_after_sort(mut nums: Vec<i64>) {
    // traverse
    let len = nums.len();
    nums.sort();

    for i in 0..len {
        if nums[i] < 50 {
            nums[i] = 0;
        }
    }
}

use rand::seq::SliceRandom;
use std::time;

#[test]
fn test_instruction_cache_hit() {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i64> = (1..100).collect();
    nums.shuffle(&mut rng);
    println!("we got nums = {:?}", nums);
    let nums_cloned = nums.clone();

    let start = time::Instant::now();
    traverse_before_sort(nums);
    println!("traverse nums before sort it consume {:?}", start.elapsed());

    let start = time::Instant::now();
    traverse_before_sort(nums_cloned);
    println!("traverse nums after sort it consume {:?}", start.elapsed());
}