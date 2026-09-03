// in this normal var is immutable 
// rust is a block-scoped lang


pub fn run() { 
    let name = "brad";
    let age = 20; 
    println!("my name is {} and my age is {}" ,name , age)

    let mut demon = 10; 
    demon = 50;  // this is mutable 
    println!("{}", demon) ;

    // def const not use that much 
    const ID:i32 = 001; 
    println!("ID = {}",ID); 

    // assing multiple vars
    let (my_name , my_age) = ("brad" , 37);

}