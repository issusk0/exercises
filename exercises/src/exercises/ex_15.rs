#[derive(Debug)]

pub struct Item{
    pub value: i32,
}

pub fn ex_15(v: &mut Vec<Item>){


    //divide the vector in two slices to get the last and first element with a valid memory
    //pointers
    let len = v.len();
    let (l, r) = v.split_at_mut(len - 1);


    let last_r = &mut r[0];
    last_r.value -= 50;

    let f_left = &mut l[0];

    f_left.value += 50; 
    
    println!("Vector: {:#?}",v);
}



pub fn data() ->Vec<Item>{
    let vector_data = vec![Item{value:1}, Item{value:2}, Item{value:3}, Item{value:4}];
    vector_data
}
