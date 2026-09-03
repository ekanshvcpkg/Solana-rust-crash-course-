pub fn run() { 
    println!("hellow demon");

    // basic formating 
    println!("{}is from good place{}", "demon", "base"); 

    // positonal arguments 
    println!("{} is from {} and he like to {2} and he is {0}" , "brad", "handwani", "coding"); // inside it is indexing 

    // name arguments 
    println!("{name} is a good boy but {bad}", name = "demon" , bad = "ooono");

    // placeholder traiots 
    println!("Binary:{:b}  hex:{:x} octal:{:o}",  10 , 10 , 10 );

    // placeholder for debug traiots 
    println!("{:?}" , (13 , "hellow" , true));

    // basic maths 
    println!("10+10 = {}", 10 + 10 );
}
