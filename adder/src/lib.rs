



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[should_panic(expected="failed!")]
    #[test]
    fn test(){
         panic!("afailed!!", );
    }

    #[test]
    fn larger_can_hold_small() {
        let large = Rectangle{
            width: 8,
            height: 7,
        };

        let small = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(large.can_hold(&small),"{:?} can hold {:?}",large,small);
    }

    #[test]
    fn small_cannot_hold_large() {
        let large = Rectangle{
            width: 8,
            height: 7,
        };

        let small = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!small.can_hold(&large),"{:?} can not hold {:?}",small,large);
    }
    
    #[test]
    fn rectangle_equal(){
        let large = Rectangle{
            width: 8,
            height: 7,
        };

        let small = Rectangle {
            width: 5,
            height: 1,
        };

        assert_ne!(large,small);
    }

    #[test]
    fn it_add_two(){
        assert_eq!(4,add_two(2),"{} add 2 failed",2);
    }

    #[test]
    #[ignore]
    fn test_result() -> Result<(),String> {
        if 2+2 == 5{
            Ok(())
        }else{
            Err(String::from("not equal!"))
        }
    }
}

#[derive(Debug,PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other : &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32{
    println!("hello:addtwo");
    a + 2
}
