


use std::fmt;


#[derive(Debug)]
pub struct Item{
    pub value: i32
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Item {{ value: {} }}", self.value)

    }
}
pub fn exercise_6(){
    // tu código aquí
    let mut data = Vec::new();
    data.push(Item{value: 1});
    data.push(Item{value: 2});
    let reference = &data[0];
    println!("Reference value: {}", reference);
    data.push(Item{value: 3});
    println!("First element: {}", data[0]);

}
