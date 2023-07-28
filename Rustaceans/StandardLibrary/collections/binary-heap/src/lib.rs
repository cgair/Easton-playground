#![allow(unused)]
// Credit to <https://doc.rust-lang.org/src/alloc/collections/binary_heap.rs.html#268>

/// A priority queue implemented with a binary heap.
/// 
/// This will be a max-heap
pub struct MyBinaryHeap<T> {
    data: Vec<T>
}

impl<T: Ord + Default> MyBinaryHeap<T> {
    /// Creates an empty `BinaryHeap` as a max-heap.
    ///
    pub fn new() -> Self {
        MyBinaryHeap { data: vec![T::default()] }
    }

    /// Pushes an item onto the binary heap.
    ///
    pub fn push(&mut self, item: T) {
        self.data.push(item);
        let len = self.len();
        self.sift_up(len);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    fn sift_up(&mut self, pos: usize) {
        // while pos > 1 && self.data[]
    }
}


#[test]
fn test_1() {
    let nums = vec![1, 1, 2, 2, 3, 4, 3];
    let mut ret = 0;
    let _ = nums.iter().map(|&x| { ret ^= x }).collect::<Vec<_>>();
    assert_eq!(ret, 4);
}

#[test]
fn test_2() {
    let nums = vec![1, 1, 2, 2, 3, 4, 3, 5];
    let mut ret = 0;
    let _ = nums.iter().map(|&x| { ret ^= x }).collect::<Vec<_>>();
    println!("{:#034b}", ret);

    assert_eq!(location(2), 2);
    assert_eq!(location(4), 3);
    assert_eq!(location(8), 4);
    let n = location(ret as u32);
    // println!("{n}");
    for n in nums {
        if n >> 0 & 1 == 1 { println!("{n}") }
    }

}

fn location(mut num: u32) -> usize {
    let mut n = 0;
    while num != 0 {
        n += 1;
        num = num >> 1;
    }
    n as usize
}