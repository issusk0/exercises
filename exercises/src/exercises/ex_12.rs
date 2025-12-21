#[derive(Debug)]
pub struct Item {
    pub value: i32,
}

pub fn ex_12() {
    // tu código aquí
    let mut vector = vec![
        Item { value: 1 },
        Item { value: 2 },
        Item { value: 3 },
        Item { value: 4 },
    ];


    for i in vector.iter_mut(){
        if i.value % 2  ==0{
            i.value += 10;
        }else{
            i.value +=1;
            
        }
    }

    println!("Vector: {:#?}", vector)
}

