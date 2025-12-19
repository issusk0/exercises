#[derive(Debug)]
pub struct Item { pub value: i32 }

pub fn ok_nll() {
    // debe compilar
    let mut vector = Vec::new();
    vector.push(Item { value: 1 });
    vector.push(Item { value: 2 });
    let r = &vector[0];
    println!("{:?}", r);
    vector.push(Item { value: 3 });
    println!("{:?}", vector[0]);
}

pub fn fail_borrow() {
    // debe NO compilar (y tú me explicas por qué)

    /*
    esto no compila debido a que la referencia esta siendo "prestada" como inmutable en r
    luego al hacerle un push, se modifica el borrow como inmutable a mutable, haciendo que indirectamente
    el valor de r cambie, y su borrow igual(segun lo que entendi) para solucionar esto hay que crear una instancia
    en la que r tenga un uso exclusivo de mutabilidad para que no haya problemas debido a que solo
    puede haber 1 borrow por variable (mutable o inmutable)
     */
    let mut vector = Vec::new();
    vector.push(Item { value: 1 });
    vector.push(Item { value: 2 });
    vector.push(Item { value: 3 });
    let r = &vector[0];
    println!("{:?}", r);
}
