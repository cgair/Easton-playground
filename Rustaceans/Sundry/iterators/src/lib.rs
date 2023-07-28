

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![0, 1, 2, 3, 4];
        let mut v1_iter = v1.iter();    // Calling the next method consumes, or uses up, the iterator.
        
        assert_eq!(v1_iter.next(), Some(&0));   // Each call to next eats up an item from the iterator. 
        assert_eq!(v1_iter.next(), Some(&1));    
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), Some(&4));
        assert_eq!(v1_iter.next(), None);

        let v2 = vec![0, 1, 2, 3, 4];
        let v2_iter = v2.iter();    // We didn’t need to make v2_iter mutable when we used a for loop because the loop took ownership of v2_iter and made it mutable behind the scenes.
        
        for v in v2_iter {
            println!("Got {}", v);
        }
    }
}
