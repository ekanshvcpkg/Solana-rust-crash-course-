// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own
// string data


pub fn run () { 
    // these func only work in string::form 
    let mut  demon = String::from("demon hex ");

    // length of the string 

    println!("len of the line->{}" , demon.len());

    // push char in the string 
    demon.push('h');

    // push the string 
    demon.push_str("hell yea");

    //capacity in bytes 

    println!("Cap-{}", demon.capacity());

    // is empty 
    println!("Is Empty-{}" , demon.is_empty()); 

    // contain words 
    println!("Contain Word->(hex){}", demon.contains("hex"));


    // replace 
    println!("Replace hex-{}", demon.replace("hex", "bytes")) ; 

    // loop thorouh string using white space 

    for anyvarname in demon.split_whitespace(){ 
        println!("{}" , anyvarname);
    }

    // Assertion testing you can say === 
    assert_eq!(5,demon.len());

    // 

    


    

}