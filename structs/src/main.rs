

fn main() {
    let w = 30;
    let h  = 50;

    println!("area: {}", area(w, h));

    let rect = Rectangle{ 
        width : 4,
        height :100
    };

    let rect2 = Rectangle{ 
        width : 5,
        height :6
    };

    println!("area2: {}", area2(&rect));

    println!("rect: {:?}",rect);
    println!("rect: {:#?}",rect);

    println!("area3: {}", rect.area());

    println!("fit:{}",rect.fit(&rect2));

    println!("rect: {:#?}",Rectangle::square(100));
}

#[derive(Debug)]
struct Rectangle{
    width : u32,
    height : u32
}

impl Rectangle{
    fn area(&self) -> u32{
        self.height*self.width
    }

    fn fit(&self,other : &Rectangle)->bool {
        self.width>other.width && self.height>other.height
    }

    fn square(s : u32) -> Rectangle{
        Rectangle{
            width:s,
            height:s,
        }
    }
}

fn area(width: u32, height: u32)-> u32{
    width * height
}



fn area2(rect : &Rectangle) -> u32{
    rect.width* rect.height
}
