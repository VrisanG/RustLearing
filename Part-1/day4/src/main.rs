// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {rect1:?}");
// }
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter
}
fn main(){}
fn value_in_cents(coin:Coin)->u8{
    match coin{
        Coin::Penny=>{
            println!("LOL your a brookie");
            1
        },
        Coin::Nickel=>5,
        Coin::Dime=>10,
        Coin::Quarter=>25,
        
    }
}