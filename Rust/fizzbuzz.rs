fn main() {
    // for i in 1..101 {
    //     if (i % 3) == 0 && (i % 5) == 0 {
    //         println!("fizzbuzz");
    //     }
    //     else if (i % 3) == 0 {
    //         println!("fizz");
    //     }
    //     else if (i % 5) == 0 {
    //         println!("buzz");
    //     }
    // }
    
    for i in 1..101 {
        // match i % 3 {
        //     0 => println!("fizz"),
        //     _ => println!("{}", i)
        // }
        match i % 5 {
            0 => println!("buzz"),
            _ => println!("{}", i)
        }
    }
}
