fn main() {
    let mut v = vec![1, 2, 3];
    let first_element = v.get_mut(0);
    match first_element {
        Some(element) => {
            *element = 10;
            println!("Value at the first element of v: {:?}", v[0]);
        }
        None => {
            println!("Element not found");
        }
    }
}