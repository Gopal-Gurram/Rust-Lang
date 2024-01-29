fn main() {

    // Create a empty vector
    let mut a = Vec::new();
    let mut b = vec![];

    //create a vectory with data type

    let mut a2: Vec<i32> = Vec::new();
    let mut b2: Vec<i32> = vec![1,2,3];
    let mut c2 = vec![ii32,2,3]; // Suffixing first value with data type
    
    let mut a3 = vec![1,2,3];
    let mut b3: Vec<i32> = vec![1,2,3];
    let mut c3 = vec![1i32,2,3]
    let mut d4 = vec![0;10]; // Ten Zeros

    //Access and change data

    let mut c = vec![1,2,3];
    c[0]=2;
    c[1]=4;
    //c[6] = 2; Cannot assign values this way, index out of bounds
    println!(":?",c); // c= [2,4,3]

  //push and pop
    let mut d: Vec<i32> = Vec::new();
    d.push(1); //[1] : Add an element to the end
    d.push(2); //[1, 2]
    d.pop(); //[1] : : Remove an element from the end

    // 🔎 Capacity and reallocation
let mut e: Vec<i32> = Vec::with_capacity(10);
println!("Length: {}, Capacity : {}", e.len(), e.capacity()); //Length: 0, Capacity : 10

// These are all done without reallocating...
for i in 0..10 {
    e.push(i);
}
// ...but this may make the vector reallocate
e.push(11);

// Vectors can be used with iterators in three ways, Vectors can be used with iterators in three ways,
let mut v = vec![1, 2, 3, 4, 5];

for i in &v {
    println!("A reference to {}", i);
}

for i in &mut v {
    println!("A mutable reference to {}", i);
}

for i in v {
    println!("Take ownership of the vector and its element {}", i);
}

}
