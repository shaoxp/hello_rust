fn main() {
    println!("Hello, world!");

    let msg = Message::ChangeColor(Color::Rgb(256,0,0));
    match msg{
        Message::ChangeColor(Color::Rgb(r,g,b)) => {
            println!("change to color to rgb({},{},{})",r,g,b);
        }
        Message::ChangeColor(Color::Hls(h,l,s)) => {
            println!("change to color to hls({},{},{})",h,l,s);
        }
        _ =>{},
    }

    let ((feet,inches),..)=((3,10),Point {x:3,y:-10},Point {x:1,y:-2});

    println!("f:{},i:{}",feet,inches);

    let s = Some(String::from("hello"));

    if let Some(_) = s{
        println!("sucess bind");
    }

    println!("used after move {:?}", s);

    let num = (1,2,3,4,5,"abc");

    match num {
        (f,..,e) => {
            println!("f:{},e:{}",f,e);
        }
    }

    match num {
        (_,s, ..)=> {
            println!("{}",s);
        }
    }

    let msg = Message::Hello{
        id:5
    };

    match msg{
        Message::Hello{
            id:id@ 3..=7
        }=>println!("found in 3,7{}",id),
        Message::Hello{
            id: 10..=12
        } => println!("found in another"),
        Message::Hello{
            id
        }=>println!("default :{}",id),
        _ =>{}
    }

    let s1: &str= "abc";
    let s2: Box<str> = Box::new(s1);
}

enum Color {
    Rgb(i32,i32,i32),
    Hls(i32,i32,i32),
}

enum Message{
    Quite,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(Color),
    Hello { id: i32},
}

struct Point{
    x: i32,
    y: i32,
}

fn infinit()->!{

    loop{
        println!("(abc)", );

        if 3==3{
            panic!("abc", );
        }
    }
}