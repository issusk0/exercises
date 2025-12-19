#[derive(Debug)]
pub struct Item { pub value: i32 }

pub fn exercise_10() {
    // requisitos abajo
    let mut vector = vec![Item { value: 2 }, Item { value: 3 }, Item { value: 4 }];
    {
        let first_item = first_item_mut(&mut vector);
        match first_item {
            Some(first_item) => {
                first_item.value = 100;
            },
            None => {
                println!("empty vector")
            }
        }
    }
    println!("Vector: {:?}", vector);
}


pub fn first_item_mut(v: &mut Vec<Item>) -> Option<&mut Item>{
    v.first_mut()
}
