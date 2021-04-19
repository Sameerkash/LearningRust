// fn main() {
//     let a_number = 10;
//     let a_boolean = true;



//     println!("the number is {}.", a_number);
//     println!("the boolean is {}.", a_boolean);
//   }

// Shadowing 

// fn main() {
//     let number = 5;          // the first binding is created using the name "number"
//     let number = number + 5; // a different binding shadows the name "number"
//     let number = number * 2; // again, a new binding is created
//     println!("The number is: {}", number);
// }

//types
// fn main() {
//     // Addition
//     println!("1 + 2 = {}", 1u32 + 2);

//     // Subtraction
//     println!("1 - 2 = {}", 1i32 - 2);
//     // ^ Try changing `1i32` to `1u32` to see why the type is important

//     // Integer Division
//     println!("9 / 2 = {}", 9u32 / 2);

//     // Float Division
//     println!("9 / 2 = {}", 9.0 / 2.0);

//     // Multiplication
//     println!("3 * 6 = {}", 3 * 6);

//     let is_bigger = 1 > 4;
//     println!("{}", is_bigger);

//     let c = 'z';
//     let z = 'ℤ';
//     let heart_eyed_cat = '😻';  // prints "false"
// }

ad