
    let a: [i32 , 4] = [1,2,3,4]; // parent array
    let b: &i32 = &a // slicing whole array

    let c = &a[0..4]; // from 0th position to 4th(excluding)
    let d = &a[..]; //slicing the whole array
    let e = &a[1...]; //[2,3,4]
