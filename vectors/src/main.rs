use std::vec;

fn main() {
    //default constructor of an empty vector with datatype "i32".
    //it is still updatable, as it is stored on the heap and is mutable.
    //a vector can only hold one datatype at a time, however it can hold any datatype.
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

    //for loop using an immutable reference to each element in the vector
    for i in &v {
        println!("{i}");
    }

    //for loop using mutable references to each element in the vector
    //the "*"(dereference) operator will be discussed later
    for i in &mut v {
        *i *= 2;
        println!("{i}");
    }

    //enums can be used to represent multiple datatypes, as the enum itself is considered one type,
    //however all the different variants of the same enum, is stored as the same type.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    //compiletime checks will ensure safety by only letting a vector handle one type at a time,
    //and then using enums for multitype types. This is done with a match statement.
    //enums are of course limited in that you have to specify everything at compiletime.
    //later on a trait object will be covered, to solve this issue.
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("shark")),
        SpreadsheetCell::Float(69.69),
    ];

    //all the values in the vector are dropped once it goes out of scope
    {
        let _v2 = vec![1, 2, 3];

        //use vector for cool logic and coding
    } //<- everything goes out of scope here and are feed in memory
}
