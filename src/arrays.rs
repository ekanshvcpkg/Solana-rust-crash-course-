

// fix list which has the same data types 
pub fn run () { 

    let numbers:[i32; 5 ] = [1,2,3,4,5];  // this need to be fix size if not error 
    // fix type if not error 

    // if mut 
    let mut numbers_but_mut:[i32; 5 ] = [1,2,3,4,5]; 

    println!("{:?}", numbers);

    // to get the single value 
    println!("{}" , numbers[0]);
    println!("{}", numbers_but_mut.len());


    // arrays are stack allocated 
    println!("array ocp there may bytes ->{}", std::mem::size_of_val(&numbers)); // &numbers is a reference. More precisely, an immutable reference in Rust

    //get slice 

    let slice: &[i32] = &numbers[1..3]; // give from index 1 to 2 
    // ! numbers[1..2] → creates a slice from index 1 up to (but not including) 2
    println!("slic:{:?}", slice);

    // 





}