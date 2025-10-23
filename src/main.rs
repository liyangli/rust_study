// use std::cmp::Ordering;
// use std::io;

// use rand::Rng;
mod chart5;

fn main() {
    chart5::chart5_1::main();

//     println!("Guess the number!");
//     let secret_number = rand::rng().random_range(1..=100);
//     println!("The secret number is: {secret_number}");

//     loop {
//         println!("Please input your guess.");

//         let mut guess = String::new();

//         io::stdin().read_line(&mut guess).expect("Failed to read line");
        
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("Please type a valid number!");
//                 continue;
//             }
//         };
        
//         println!("You guessed: {guess}");
        
//         match guess.cmp(&secret_number){
//             Ordering::Less => {
//                 println!("Too small!");
//                 continue;
//              }
// 	    Ordering::Greater =>{
//                 println!("Too big!");
//                 continue;
//             }
// 	    Ordering::Equal => {
// 	            println!("You win!");
//                 break;
//             }
//         }
//     }
//     let x = 5;
//     println!("The value of x is : {x}");
// //    x =6;
//     let mut num = 10;
//     println!("The value of number is :{num}");
//     num = 12;
//     println!("The value of number after is:{num}")

    // clone_demo();
    // clone_demo2();
    // let s = String::from("hello");
    // takes_ownership(s);
    // // 这里s已经失效了，所以不能使用
    // // println!("s is :{s}");
    // let x = 5;
    // makes_copy(x);
    // println!("x is :{x}");

    // let s1 = give_ownership();

    // let s2 = String::from("hello");
    // let s3 = takes_and_gives_back(s2);
    // println!("s3 is :{s3}");

    // let s1 = String::from("hello liyangli");
    // let len = calculate_length(&s1);
    // println!("The length of {} is {}", s1, len);

    // let mut s2 = String::from("hello");
    // change(&mut s2);
    // println!("s2 is :{s2}");
}


fn change(some_string: &mut String){
    some_string.push_str(" liyangli");
}


/**
 * 通过指针引用方式，防止值的所有权被转移
 */
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn give_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(s: String){
    println!("s is :{s}");
}

fn makes_copy(x: i32){
    println!("x is :{x}");
    // x = x +4;
}


fn clone_demo(){
    let s1 = String::from("hello");
    println!("s1 is :{s1}");
    let s2 = s1;
    println!("s2 is :{s2}");
    // println!("s1 is :{s1}");
    let s3 = s2.clone();
    println!("s3 is :{s3}");
    println!("s2 is :{s2}");
    
}

fn clone_demo2(){
    let x = 5;
    let y = x;
    println!("x is :{x}");
    println!("y is :{y}");
}
