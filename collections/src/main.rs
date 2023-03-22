fn main() {
    //default constructor of an empty vector with datatype "i32".
    //it is still updatable, as it is stored on the heap and is mutable.
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    //infering datatype as "i32" (the standard integer type),
    //by adding values in the constructor. This is using the "!vec" macro
    let v1 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v1[2];
    println!("The third element of the vector v1 is {third}");

    let fourth: Option<&i32> = v.get(3);
    match fourth {
        Some(fourth) => println!("The fourth element of the vector v is {fourth}"),
        None => println!("There is no fourth element."),
    }
}
