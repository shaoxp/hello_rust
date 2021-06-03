use restaurant::front_of_house::hosting;
use restaurant::back_of_house;

fn main() {
    println!("Hello, world!");

    restaurant::eat_at_restaurant();
    hosting::add_to_waitinglist();
    let a = back_of_house::Breakfast::summer("good");
}
