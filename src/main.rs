use std::io;
fn main() {
    let xstring = String::from("Hello World");
    println!("object String: {}",xstring);
    println!("the prelude is the list of things that rust automatically imports");
    println!("for example:fn,println. Input_output we use(import) from std::io");
    println!("std is standard package; io is module. :: etc. ");
    let mut xinput_obj = String::new(); // new Object
    println!("enter something");
    // creating mutable refenence(&) and catching errors
    io:: stdin().read_line(&mut xinput_obj)
      .expect("failed to read line");
    println!("just now you have entered: {}", xinput_obj);

    let mut xinputObj = String::from("something");
    println!("before input line");
    io:: stdin().read_line(&mut xinput_obj).
       expect("failed to read line");
}
