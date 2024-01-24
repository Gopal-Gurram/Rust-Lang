// Named function before using clousers
// fn main() {
    
//     let x = 2;
//     println!("{}",get_square_value(x));
// }
// fn get_square_value(a: i32) -> i32 {

//     a * a
// }

// With optional type declaration of input and return types

// fn main(){
//     let x = 5;

//     let square = |i : i32| -> i32 {
//         i * i
//     };
//     println!("{}",square(x));
// }

/// Without type declaration of input and return types

// fn main(){
//     let x = 5 ;
//     let square = |i| i * i ;
//     println!("{}",square(x));
// }

// With optional type declarations; Creating and calling together

// fn main(){
//     let x = 15;
//     let square = |i : i32| -> i32 {i * i}(x);
//     println!("{}",square);
// }

//Without optional type declarations; Creating and calling together

fn main(){
    let x = 20;
    let square = |i| -> i32 {i * i}(x);
    println!("{}",square);
}


