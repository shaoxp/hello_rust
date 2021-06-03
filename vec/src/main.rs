fn main() {
    println!("Hello, world!");

    let mut v = vec![8,2,3,5,6];
    v.sort();
    let third = &v[2];
    println!("{}",third);

    let gv = v.get(2);
    match gv {
        Some(val)=>println!("{}",val),
        None =>println!("none"),
    }

    for i in &v {
        println!("{}",i);
    }

   for i in & mut v{
       println!("{}",i);
       *i = 5;
   }

   for i in &v {
     println!("{}",i);
   }   

   let row = vec![
       SpreadSheetCell::Float(1.25f32),
       SpreadSheetCell::Text(String::from("hello")),
       SpreadSheetCell::Int(35),
   ];

}

enum SpreadSheetCell{
    Int(i32),
    Float(f32),
    Text(String)
}
