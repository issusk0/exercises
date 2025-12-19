#[derive(Debug)]
pub struct Item {
    value: i32
}

fn make_and_get_first() -> Item {
    let v =  Item { value: 7 };
    v
}

