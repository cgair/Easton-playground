/**
 * Traverse a two-dimensional array in the following two forms.
 * Which form is the most efficient? Why?
 */

// array[N][N]
pub(crate) fn traverse1(mut nums: [[i64;8];8]) {
    let n = nums.len();
    for i in 0..n {
        for j in 0..n {
            nums[i][j] = 0;
        }
    }
}

pub(crate) fn traverse2(mut nums: [[i64;8];8]) {
    let n = nums.len();
    for i in 0..n {
        for j in 0..n {
            nums[j][i] = 0;
        }
    }
}

use std::time;

#[test]
fn test_data_cache_hit() {
    // L1 cache = 64 bytes
    // let nums1 = vec![vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 3, 4, 5, 6, 7, 8, 9], vec![3, 4, 5, 6, 7, 8, 9, 10], vec![4, 5, 6, 7, 8, 9, 10, 11], vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 3, 4, 5, 6, 7, 8, 9], vec![3, 4, 5, 6, 7, 8, 9, 10], vec![4, 5, 6, 7, 8, 9, 10, 11]];
    let nums1 = [[1, 2, 3, 4, 5, 6, 7, 8], [2, 3, 4, 5, 6, 7, 8, 9], [3, 4, 5, 6, 7, 8, 9, 10], [4, 5, 6, 7, 8, 9, 10, 11], [1, 2, 3, 4, 5, 6, 7, 8], [2, 3, 4, 5, 6, 7, 8, 9], [3, 4, 5, 6, 7, 8, 9, 10], [4, 5, 6, 7, 8, 9, 10, 11]];
    let nums2 = nums1.clone();
    let start = time::Instant::now();
    traverse1(nums1);
    println!("traverse nums1 consume {:?}", start.elapsed());

    let start = time::Instant::now();
    traverse1(nums2);
    println!("traverse nums2 consume {:?}", start.elapsed());
}