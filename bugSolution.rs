fn main() {
    let mut v = vec![1, 2, 3];
    // Safe way to modify the vector. 
    v[0] = 10;
    println!("v: {:?}", v);
    // Alternative approach using iter_mut
    for i in v.iter_mut() {
        *i *= 2;
    }
    println!("v: {:?}", v);
} 