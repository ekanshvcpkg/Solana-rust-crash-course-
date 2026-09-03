/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays // fix length 
*/

// Rust is a statically typed language, which means that it must know the types of all
// variables at compile time, however, the compiler can usually infer what type we want to use
// based on the value and how we use it.




pub  fn  run() {

    // def it will be "i32"
    let a = 1; 

    // def float will be f64 
    let b = 2.34; 

    // addding size by your self 
    let c: i128 = 234; 

    // find max size

    println!("Max for i32 {}", std::i32::MAX); 
    println!("Max for i64 {}" , std::i64::MAX);

    // bool 
    let is_active = true; 
    let is_notactive = false;

    println!("{:?}" , (a , b , c ,is_active));

    // get bool from expression 

    let is_greator = 10<5;

    println!("{}",is_greator);

    // char 

    let g = 'a';

    println!("{}", g)





    
}