#[derive(Debug)]
pub struct Item { pub value: i32 }

pub fn exercise_9() {
    // requisitos abajo
    let mut empty_vector: Vec<Item> = vec![];
    first_item(&empty_vector);
    empty_vector.push(Item { value: 1 });
    first_item(&empty_vector);
}

pub fn first_item(v: &Vec<Item>) -> Option<&Item>{
    for i in v {
        println!("Found item: {}", i.value);
        return Some(i);
    }
    println!("No first item in vector");
    None
}

