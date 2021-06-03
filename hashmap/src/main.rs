use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut s = HashMap::new();
    s.insert(String::from("Blue"), 1.0);
    s.insert(String::from("Yellow"),2f32);

    let t = vec![String::from("abc"),String::from("xyz")];
    let v = vec![1,2];
    let mut ss : HashMap<_,_> = t.into_iter().zip(v.into_iter()).collect();

    let a = String::from("ABC");
    let b = String::from("XYZ");

    let mut m = HashMap::new();
    m.insert(a, b);

    // println!("{},{}",a,b);

    let res = s.get("Blue");

    if let Some(val)  = res {
        println!("{}",val);
    }

    match res {
        Some(vv) => println!("{}",vv),
        None =>println!("None"),
    }

    for (k,mut vvv) in &mut ss{
        *vvv+=2;
        println!("{},{}",k,vvv);
    }

    for (k, vvv) in &ss{
        println!("{},{}",k,vvv);
    }

    ss.insert(String::from("abc"), 6);
    ss.entry(String::from("abc2")).or_insert(8);

    println!("{:?}",ss);

    update_old_value();
}

fn update_old_value(){
    let text = "hellow world wonderful  world";
    let mut m = HashMap::new();
    for wd in text.split_whitespace(){
        let count = m.entry(wd).or_insert(0);
        *count+=  1;
    }

    println!("{:?}", m);
}
