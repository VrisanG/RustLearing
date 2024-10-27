// use std::io;

// fn main(){
//     let mut given_number= String::new();

//     println!("Enter the Digit you want to check ");

//     io::stdin().read_line(&mut given_number).expect("Not able to read the number");
    
//     let given_number= given_number.trim().parse::<u32>().expect("The input value is not a number");
//     is_even(given_number);

// }
// fn is_even(number:u32){
//     println!("Enter the Digit you want to check ");
//     if number%2==0{
//         print!("The given number is Even:True")
//     }else {
//         print!("The given Number is Not divisible to 2")
//     }

// }

use std::io;

fn main(){
    let mut number=String::new();
    
    println!("Enter the number you want to have the Fibonacci to");
    
    io::stdin().read_line(&mut number).expect("Failed to get the Number you specified");

    let number = number.trim().parse().expect("The value you provided is not a number");

    fib(number)

}
fn fib(num:u32){
    let mut first = 0;
    let mut second = 1;
    if num==0{
        println!("{first}")
    }
    if num==1 {
        println!("{first}")
    }
    for i in 1..num -1{
        let temp = second;
        second= second +first;
        first=temp;
    }

    println!("The Fibonacci till the {num} will be {second}  ");
}
fn get_string_length(s: &str) -> usize {
    s.chars().count()
}

// fn main(){
//     let my_string= String::from("Heloo world ");
//     let lenght = get_string_length(&my_string);
//     println!("{lenght}")
// }
