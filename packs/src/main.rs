
fn main() {
    
    let v: Vec<i32> = Vec::new();
    let mut v0: Vec<i32> = vec![1,2,3,4,5];

    v0.pop();

    println!("Capacity: {}", v0.capacity());
    println!("Lenght: {}", v0.len());
    
}
