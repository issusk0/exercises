#[derive(Debug)]
//same here(pub/priv)
pub struct Item {
    value: i32,
}
//doesnt compile due to public private
pub fn make_items() -> Vec<Item> {
    let mut v = Vec::new();
    v.push(Item { value: 42 });
    v
}
//same here(pub/priv)
pub fn get_first(v: &Vec<Item>) -> &Item {
    &v[0]
}

