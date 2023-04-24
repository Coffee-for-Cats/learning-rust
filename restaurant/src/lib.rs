mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        //Remember, it's not a method, it's just a function that are related to Breakfast.
        //You can see it because the first parementer isn't "self".
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

mod front_of_house;

//It just adds the "nickname" hosting.
//I need to use crate:: because it is a bin?
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    //front_of_house function.
    hosting::add_to_waitlist();

    //back_of_house function test.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    //I cannot access this field, because it is not public!
    //meal.seasonal_fruit = String::from("Bluebarries");
}
