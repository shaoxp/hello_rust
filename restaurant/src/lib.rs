pub mod front_of_house;

pub use front_of_house::serving::back_of_house;

pub fn eat_at_restaurant(){
    crate::front_of_house::hosting::add_to_waitinglist();

    front_of_house::hosting::add_to_waitinglist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    // let meal2 = front_of_house::serving::back_of_house::Breakfast{
    //     toast:String::from("abc")
    // }

    println!("eat {}",meal.toast);
    // meal.season_fruit = String::from("Blue");

    let order1= front_of_house::serving::back_of_house::Appetizer::Salad;
}