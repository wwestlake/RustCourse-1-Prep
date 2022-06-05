
fn borrow(value: &str) {
    println!("{}", value)    
}


fn main() {
    borrow("This is a string slice");
}
