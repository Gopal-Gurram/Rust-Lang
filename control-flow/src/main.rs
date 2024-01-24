fn main() {
    vote();
    loop_counter();
    condition();
    match_checking();
}
fn condition(){
    let condition = true ;
    let number = if condition {5} else {6};
    println!("The value of number is : {number}");
}
fn vote(){
    let age = 18 ;

    if age < 19 {
        println!("You are eligible to vote");
    } else {
        println!("You are not eligible to vote");
    }
}
// Loops

fn loop_counter() {
    let mut counter = 0 ;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The value of counter is : {result}");
}

// Match

fn match_checking() {
    let tshirt_width = 20;
    let tshirt_size = match tshirt_width {
        16 => "S",// small 16
        17 | 18 => "M" ,
        19..= 21 => "L",
        22 => "XL",
        _=> "Not Available",
    };
    println!("{}" , tshirt_size);
}




