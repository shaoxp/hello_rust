fn main() {
    println!("Hello, world!");

    let c = CustomerPointer(String::from("abc"));
    let c = CustomerPointer(String::from("efg"));
    let d = CustomerPointer(String::from("hik"));

    println!("customer point created!");
    drop(c);
    let a = &c.0;
    println!("droped before the end of main");
}

struct CustomerPointer(String);

impl Drop for CustomerPointer{

    fn drop(&mut self) {
         println!("Droping with data {}",self.0);
    }
}
