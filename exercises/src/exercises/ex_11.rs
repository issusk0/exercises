#[derive(Debug)]

pub struct Item{
    pub value: i32
}


pub fn ex_11(){

    let mut vector = vec![Item{value: 1}, Item{value: 2}, Item{value:3}];
    let (l, r) = vector.split_at_mut(2);
    let first = &mut l[0];
    let third = &mut r[0];

    first.value += 10;
    third.value += 20;
    println!("Vector: {:?}", vector);

}
