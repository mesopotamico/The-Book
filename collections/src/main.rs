fn main() {
    let s1: String = String::from("hello");
    let s2: String = String::from("world!");

    let hw = format!("{s1} {s2}");


    println!("{}", hw);
    println!("{}", s1);
    println!("{}", s2);
    println!("The first letter of s1 is {}", s1[0]);
}
