fn main() {
    println!("Hello, world!");
    let mut s = 2.to_string();

    println!("{}",s);   

    s.push_str(" abc中国");
    s.push('\t');
    s.push('d');

    println!("{}",s);


    let o = "xyz";
    s.push_str(o);
    println!("{}",o);

    for c in s.chars(){
        if c == '中'{
            println!("{}",c)
        }
    }

}
