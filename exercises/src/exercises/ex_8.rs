#[derive(Debug)]
pub struct Item { pub value: i32 }

pub fn exercise_8() {
    // requisitos abajo
    let mut vector = Vec::new();
    vector.push(Item { value: 1 });
    vector.push(Item { value: 2 });
    vector.push(Item { value: 3 });
    println!("Whole vector is: {:?}", vector);
    let first_element_reference = &mut vector[0].value;
    println!("The first element is {:?}", first_element_reference);
    *first_element_reference += 10;
    println!("First element + 10: {:?}", first_element_reference);


}
