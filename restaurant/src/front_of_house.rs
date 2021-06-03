   pub mod hosting;

    pub mod serving{
        fn take_order(){}
        fn serve_order(){}
        fn take_payment(){}
        pub mod back_of_house{
            fn fix_incorrect_order(){
                cook_order();
                super::serve_order();
            }

            fn cook_order(){}

            pub struct Breakfast{
                pub toast:String,
                season_fruit:String,
            }

            impl Breakfast{
                pub fn summer(toast:&str) ->Breakfast{
                    Breakfast{
                        toast: String::from(toast),
                        season_fruit: String::from("peaches"),
                    }
                }
            }

            pub enum Appetizer{
                Soup,
                Salad,
            }
        }
    }
