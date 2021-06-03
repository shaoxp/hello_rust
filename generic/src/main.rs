
fn main() {
    let v = vec![23,432,34,50,098];
    let v2 = vec![3,4,5,9,10,2];
    let c1 = vec!['a','g','e','f'];
    let c2 = "abcdefg";


    println!("largest:{}",largest(&v[2..]));
    println!("largest:{}",largest(&v2[..4]));
    println!("largest:{}",largest(&c1));

    // println!("largest:{}",largest(&String::from(c2).chars().collect()));

    let ip = Point{x:1, y:2};

    println!("{}",ip.x);
}

fn largest<T: std::cmp::PartialOrd+Copy>(v: &[T]) -> &T{
    let mut largest = &v[0];
    for n in v{
        if n > largest{
            largest = n;
        }
    }

    largest
}

struct Point<T> {
     x : T,
     y : T,
}

impl<T> Point<T>{
    fn x(&self) -> &T{
        &self.x
    }

    fn xx<U>(&self) ->U{
        self.x
    }
}