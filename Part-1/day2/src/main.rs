// fn main(){
//     let  mut x= 5;
//     println!("The value of x is:{x}");
//     x=6;
//     println!("The value of x is {x}");
// }
// fn main(){
//     const LOL:u32= 60*60*3;
//     println!("The value of fheshgf is:{LOL}");
// }
// fn main(){
//     let x= 5;
//     let x= x+1;
//     {
//         let x= x*2;
//         println!("The value of x is (inside the scope) {x}");
//     }
//     println!(" the value of x outside the scope is {x}");
// }
// fn main(){
//     let x:(i32,f64,u128)=(500,6.4,1);
//     let five_hundred= x.0;
//     let six_point_four= x.1;
//     let one =x.2;
//     println!("The value of five_hundred is {five_hundred}");
//     println!("The value of six_point_four is {six_point_four}");
//     println!("The value of one is {one}");

// }
// fn main(){
//     let a= [1,2,3,4,5,6];

// }
// use std::io;
// fn main() {
//     let a = [1, 3, 4, 5, 5, 7];
//     let mut index = String::new();
//     println!("Enter the Index you want to see");
//     io::stdin().read_line(&mut index).expect("lol your so bad");
//     let index: usize = index.trim().parse().expect("The index is required dude");

//     let finalPart=a[index];

//     println!("the given index specifies:{}",finalPart);
// }
// use std::io;
// fn main() {
//     let a = [1,2,3,4,5,565,99];
//     println!("Please enter the Index you want to review");
//     let mut index = String::new();
//     io::stdin().read_line(&mut index).expect("Failed to read the thingy");
//     let index:usize = index.trim().parse().expect("Invalid index for an Array");
//     let final_part= a[index];

//     println!("The thing your trying to review is {}",final_part);
// }
// fn main(){
//     println!("Hello, world!");
//     another_function();
// }
// fn another_function(){
//     println!("LOL hey from other fn too hahaha");
// }
// fn main(){
//     let condition= true;
//     let number= if condition {5} else {69};
//     println!("If the condition is true then the number is :{number}");
    
// }
// fn main(){
//     let mut counter =0; 
//     let result=loop{
//         counter +=1;
//         println!("{counter}");
//         if counter==10{
//             break counter*2;
//         };
//     };
//     println!("the result is {result}")
// }
// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 100;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }
// fn  main(){
//     let mut number=3;
//     while number !=0{
//         println!("{number}");
//         number -=1;
//     }
//     println!("lol")
// }
// fn main(){
//     let a = [10,20,30,40,50,60,70,80];
//     let mut index = 0;
//     while index <5{
//         println!("the value is:{}",a[index]);
//         index+=1;
//     }
// }
fn main(){
    let a =[10,20,30,40,50];
    for element in a 
}