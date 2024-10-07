fn main() {
    let numbers = vec![1,2,3,4,5,9,6,7,8];


    let largest = largest(&numbers); 
    println!("{largest}");
}

fn largest(vector: &Vec<i32>) -> &i32 {
    let mut largest = &vector[0];
    for i in vector {
        if i > largest{
            largest = i;
        }
    }

    largest

}

