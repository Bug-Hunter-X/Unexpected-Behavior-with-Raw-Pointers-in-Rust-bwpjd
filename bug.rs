fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 10;
    }
    println!("Value at the first element of v: {:?}", v[0]);
}