use std::collections::HashMap;
use std::hash::Hash;

fn get_default<K, V>(map: &mut HashMap<K, V>, key: K) -> &mut V
where 
    K: Eq + Hash + Clone,
    V: Default,
{
    match map.get_mut(&key) {
       Some(value) => value,
       None => {
        // map.insert(key.clone(), V::default());
        // map.get_mut(&key).unwrap();
        todo!()
       }
    }
}


#[test]
fn test_get_default() {
    let mut map = HashMap::new();
    map.insert(0, "AD");
    map.insert(1, "BD");
    map.insert(3, "CD");

    let v = get_default(&mut map, 0);
    println!("{}", v);
}