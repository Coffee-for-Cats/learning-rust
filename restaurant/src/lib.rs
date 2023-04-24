//This is a valid module creating method: "Inline"
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

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);

    //I cannot access this field, because it is not public!
    //meal.seasonal_fruit = String::from("Bluebarries");
}
