

#[derive(Debug)]
pub struct new{
    data_1: i32,
    data_2: i32,
    data_3: i32,
}

pub(crate) fn exercise_3() -> Vec<new>{
    let mut v = Vec::new();
    let data = new{ data_1: 1, data_2: 2, data_3: 3};
    v.push(data);
    v

}

