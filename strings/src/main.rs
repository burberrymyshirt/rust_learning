fn main() {
    //Strings are a collection in the fact that they are just a collection of bytes.
    //They do however have some already implemented operations, that are useful
    //in the context that this byte collection is used as a string of text.

    //In the absolute core form of rust, there is only one type of string,
    //which is the string slice (str), and is often used in its borrowed form (&str).
    //Other forms of strings, such as the string literal are stored in the programs binary,
    //and are therefore in reality just a string slice.

    //The String type, which is implemented in the standard libary and not in the core language,
    //is a type of string that is growable, mutable, owned and UTF-8 encoded. Slices are also UTF-8 encoded

    //In essense the String type is just a wrapper for a vector of bytes with a bit of extra sauce,
    //and therefore many of the vector operations also work on the String. Here we are creating a new mutable String:
    let mut _s = String::new();

    //It is also possible to add existing data to the string when creating it in the first place, like so:
    let data = "test contents";

    let _s1 = data.to_string();

    //this also works on a string literal directly:
    let _s2 = "more test content".to_string();

    //String::from() is equivalent to using .to_string(). This code below does the exact same thing as in s2:
    //this is just a matter of style, readability and/or preference.
    let _s3 = String::from("more test content");

    //Strings are, as mentioned, UTF-8 encoded, so any properly encoded line of text is a valid String
    //All of these are therefore valid strings:
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    //Strings can grow in size and contents can change, just like a vector
    //Here we grow a String using .push_str(), which appends "bar" to the end of "foo"
    let mut s4 = String::from("foo");
    s4.push_str("bar");

    //.push_str() takes a reference, as we might want to use the appended string in more code below:
    let mut s5_1 = String::from("foo");
    let s5_2 = "bar";
    s5_1.push_str(s5_2); //if .push_str() took ownership of s5_2, we wouldn't be able to print it.
    println!("s5_2 is {s5_2}");

    //there is also .push(), which appends just one char to the end of a String. Chars are defined with singlequotes
    let mut s6 = String::from("fooba");
    s6.push('r');

    //often we want to combine two existing Strings, which can be done so with the "+" operator:
    //This will take ownership of the s7_1 String and append s7_2 to it with a reference.
    //It means that s7_1 will no longer be a valid String after the "+" operator is used,
    //but because we only use a reference to s7_2, it will still be valid afterwards.
    //this is because the "+" operator is implemented in a way, where it won't make loads
    //of copies of itself to add two strings together, and is therefore pretty effecient.
    //the method header called when using "+" looks something like this, and explaints why it
    //needs to take ownership of s7_1 to be efficient: "fn add(self, s: &str) -> String {."
    let s7_1 = String::from("foo");
    let s7_2 = String::from("bar");
    let s7_3 = s7_1 + &s7_2; //here we take ownership of s7_1, and it can therefore no longer be used
    println!("{s7_3}");

    //this is how you add multiple strings together, and the two examples seen do the exact same thing, one just looks nicer:
    let s8_1 = String::from("tic");
    let s8_2 = String::from("tac");
    let s8_3 = String::from("toe");
    let s8_4 = s8_1 + "-" + &s8_2 + "-" + &s8_3;
    println!("{s8_4}");

    //the only differnece is the use of the "format!" macro
    let s9_1 = String::from("tic");
    let s9_2 = String::from("tac");
    let s9_3 = String::from("toe");
    let s9_4 = format!("{s9_1}-{s9_2}-{s9_3}"); //this does the same as println!, where instead of printing to the screen,
                                                //it just returns a String that is formatted in the way you wanted it to.
                                                //And it does not take ownership of anything, so all inputted strings are still valid.
    println!("{s9_4}")
}
