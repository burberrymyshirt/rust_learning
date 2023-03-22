fn main() {
    //default constructor of an empty vector with datatype "i32"
    let _v: Vec<i32> = Vec::new();

    //infering datatype as "i32" (the standard integer type),
    //by adding values in the constructor. This is using the "!vec" macro
    let _v1 = vec![1, 2, 3];
}
