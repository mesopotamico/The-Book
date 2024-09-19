fn main(){ 

    let square = Rectangle::square(30);

    let area = square.area();

    println!("The side of the square is: {}, and the area is: {}", square.height, area);
    
    let square2 = Rectangle::square(20);
    println!("The square can hold: {}",square.can_hold(&square2) );

}

struct Rectangle {
    height: u32,
    width: u32,
        
}

impl Rectangle {
    fn square(size: u32 ) -> Self {
        Self {
        width: size,
        height: size,
        }
    }
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self,rec: &Rectangle) -> bool {
        self.height > rec.height && self.width > rec.width 
    }
}
