fn main(){
    let s1: &str = "hello world";
    let s2: String = String::from("hola mundo");

   

    let long = que_necesidad(&s2);  
    println!("{} {}", s2, long);
}

fn que_necesidad(s: &String) -> usize {
    s.len()
}


