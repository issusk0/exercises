#[derive(Debug)]
pub struct Item{
    pub value: i32,
}




pub fn ex_13(){
    let mut vector = vec![Item{value:1}, Item{value:2}, Item{value:3}, Item{value:4}];

    for item in vector.iter_mut(){
        if item.value % 2 == 0{
            item.value += 100;
            break;
        }
    }

    println!("Vector: {:#?}", vector);
    
}
