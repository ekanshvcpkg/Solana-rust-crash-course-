// this is a static type lang 
// this use type Type Inference
// it don't use any garbage collector hence it inforce some strict rules like 
// if you do let name = 45 ; -> now the name is 45 so it is locked to an int it is same as saying that i32 .
 // ? prinln(1); // this will give error need to add place holder 

// mod  print; // i am using the print.rs
// mod vars;
// mod  types;
// mod  string;
// mod  tuples;
mod arrays;

fn main() {

    // let user1: String = String::from("Alice");  // ! this will become empty 
    // let user2: String = user1; // ! and now the user2 will have the value
    // println!("{}", user1);  // ! ->Rust’s ownership system protecting your memory in real-time

    // let user1 = String::from("demongod");
    // let user2 = &user1;
    // print!("{}\n",user1);
    // print!("{}",user2);
    // print::run();

    // vars::run();
    // types::run();
    // string::run();
     // tuples::run();
     arrays::run();



    
   






}
