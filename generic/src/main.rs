
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

    let s1= String::from("abcdefg aaaaa");
    let s2 = "9sdfsfdasf";
    let s3 = longest_and_prounce(s1.as_str(), s2, "abc");
    println!("returned: {}",s3);
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
}

use std::fmt::Display;

fn longest_and_prounce<'a,T>(
    x: &'a str,
    y: &'a str,
    txt: T,
)-> &'a str
where T:Display, {

    println!("announce :{}",txt);
    if x.len()>y.len(){
        x
    }else{
        y
    }
}
