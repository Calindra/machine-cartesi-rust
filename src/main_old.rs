use std::{collections::HashMap, hash::Hash};

fn get_default<'r, K, V>(map: &'r mut HashMap<K, V>, key: K) -> &'r mut V
where
    K: Hash + Eq + Copy,
    V: Default,
{
    map.entry(key).or_default()
    // if map.contains_key(&key) {
    //     match map.get_mut(&key) {
    //         Some(val) => val,
    //         None => unreachable!(),
    //     };
    // }

    // map.insert(key, V::default());
    // map.get_mut(&key).unwrap()
}


#[derive(Debug)]
struct Data {
    content: String,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            content: String::from("mybad"),
        }
    }
}

impl Data {
    fn new(content: String) -> Self {
        Self { content }
    }
}

fn print_map(map: &HashMap<&str, Data>) {
    let mut root = String::new();

    for (key, val) in map {
        root.push_str(&format!("({}, {:?}), ", key, val));
    }

    println!("HashMap: [{}]", root);
}

fn main() {
    let mut map = HashMap::new();
    map.insert("hello", Data::new("world".into()));
    map.insert("foo", Data::new("bar".into()));

    print_map(&map);

    let val = get_default(&mut map, "hello");
    assert_eq!(val.content, "world");

    print_map(&map);


    let val = get_default(&mut map, "bar");
    assert_eq!(val.content, "mybad");

    print_map(&map);

    // println!("Hello, world!");
}
