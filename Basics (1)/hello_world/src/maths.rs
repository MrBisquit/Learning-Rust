pub fn add(a: i32, b: i32)->i32 {
    return a + b;
}
    
pub fn takeaway(a: i32, b: i32)->i32 {
    return a - b;
}

pub fn multiply(a: i32, b: i32)->i32 {
    return a * b;
}

pub fn divide(a: i32, b: i32)->i32 {
    return a / b;
}

pub fn power(a: i32, b: i32)->i32 {
    return a * b;
}

pub fn square_root(a: i32)->i32 {
    let mut g1 = 0;
    let mut sqrt = a / 2;

    if(a == 0) { return 0; }

    while (g1 - sqrt) != 0 {
        g1 = sqrt;
        sqrt = (g1 + (a / g1)) / 2;
    }
    
    return sqrt;
}