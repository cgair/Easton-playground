// Consider an example where we wish to determine if a word contains three **consecutive** vowels.
// We don't need to own the string to determine this, so we will take a reference.

// pub fn three_vowels(word: &String) -> bool {
pub fn three_vowels(word: &str) -> bool {
    let mut vowels = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowels += 1;
                if vowels >= 3 { return true; }
            },
            _ => vowels = 0,
        }
    }

    false
}

#[test]
fn it_has_three_vowels() {
    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    assert_eq!(
        false,
        three_vowels(&ferris)
    );
    assert_eq!(
        true,
        three_vowels(&curious)
    );

    // This works fine, but the following two lines would fail:
    // println!("Ferris: {}", three_vowels("Ferris"));
    // println!("Curious: {}", three_vowels("Curious"));

    // 很可能你会说: 这并不重要, 无论如何我都不会使用 &'static str 作为输入 (就像我们使用"Ferris"时那样).
    // 即使忽略这个特殊的例子, 你仍然会发现使用 &str 会比使用 &String 更灵活.

    // Changing the ferris type to a String (i.e., let ferris = "Ferris".to_string();) is analogous to using clone() to get around ownership/borrowing errors.
    // Using String means the program must copy the value. When using a reference, such as &str, no copy is made (more efficient).

    let sentence_string =
        "Once upon a time, there was a friendly curious crab named Ferris".to_string();
    for word in sentence_string.split(' ') {
        if three_vowels(word) {     // 字符串切片是一个&str, 而不是一个 &String, 需要一次内存分配来转换为 &String
            println!("{} has three consecutive vowels!", word);
        }
    }

    /* Example of Deref Coercion */
    let string_literals = "Curious";

    let owned_string = string_literals.to_string();     // or String::from("Curious")
    println!("{}: {}", string_literals, three_vowels(&owned_string));
    
    let counted_string = std::rc::Rc::new(string_literals.to_string());
    println!("{}: {}", string_literals, three_vowels(&counted_string));

    let atomically_counted_string = std::sync::Arc::new(string_literals.to_string());
    println!("{}: {}", string_literals, three_vowels(&atomically_counted_string));
}
