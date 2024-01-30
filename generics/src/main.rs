
fn takes_anything <T> (x: T){ //x has a type T

}
fn takes_two_of_the_same_thing <T> (x: T,y:T){ // both x and y has same type

}
fn takes_two_things <T , U> (x:T , y:U){ //mutliple types

}

//Generalizing Structs 

struct Point <T>{
    x:T,
    y:T
}

fn main(){
    let mut point_a = Point{x:1,y:2} // {T is int type}
    let mut point_b = Point{x:0.2,y :0.4} // {T is a float type}
}

//Generalizing Enums

enum Option<T>{
    some(T),
    none,
}
enum Result<T,E>{
    ok(T),
    Err(E),
}
// Above Option and Result types are some special kind of generic types that are already defined in rust standard library
//An Option value can have some value or none value
//An Result value can have either success or error

//Usage of options
//01
fn get_username_by_id(username: &str) -> Option<usize> {
// if the username can be found in the system , set userId
return Some(userId);
// else
  None
}

// so on the above function , instead of setting useId as a return type
// set a userId as Option<usize>
// Instead of return userId return Some(userId)
// else None {Remember !} in the last return statement no need to return keyword and ending with ;

//02
struct Task{
    title:String,
    assingee:Option<Person>,
}

//Instead of assingee Person we use Option<Person>
//beacause the task has not been assigned to specefic person

// -----------------------------------------------------------

// When Using option types as a return types on functions
// We can use pattern matching to catch relevent return type(Some/None) when calling them

fn main(){
    let username = "anonymous"
    match get_username_by_id(username) {
        None => println!("User not Found"),
        Some(i)=>println!("User Id: {}",i)
    }

}

