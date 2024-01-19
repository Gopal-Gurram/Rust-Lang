fn main() {
    println!("Hello, world!");
    another_function(5);
    with_statement();
    with_expression();
    let x = five() ;
    println!("the value of x: {x}");
}
fn another_function(x : i32){
    println!("The value of x is {x}");
}
fn with_statement(){
    let x=1;
}
fn with_expression(){
    let y = {
        let x = 4;
        x + 1
    };
    println!("The value of the y is: {y}");
}
fn five() -> i32 {
    5
}