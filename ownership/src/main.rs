fn main() {
   let s1 = String::from("hello");
   let s2 = s1;

   println!("valule of s: {}",s2);

   let mut s2 = takes_ownship(s2);
   println!("valule of s: {}",s2);

   make_copy(32);

   borrow(&mut s2);

   let r1 = &mut s2;
   // s2.push_str("aaa");
   r1.push_str("aaa");
   println!("{}",s2);
 
   //let a2 = & mut s2;
  
   // s2.push_str("aaaa");
   let s = &s2;
  
   println!("{},{}",s,s2);


   let refernce_to_nothing = dangle();
   
}

fn dangle() -> String{
    let s = String::from("abc");
    s
}

fn takes_ownship(str:String) -> String{
    println!("input string: {}",str);
    str
}

fn make_copy(i : i32){
    println!("copy input: {}",i);
}

fn borrow(str: &mut String){
    str.push_str("hello!");
}
