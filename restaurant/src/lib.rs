fn _deliver_order() {}

mod front_of_house {
    pub mod hosting {
        pub fn _add_to_waitlist() {}

        fn _seat_at_table() {}
    }

    mod serving {
        fn _take_order() {}

        fn _serve_order() {}

        fn _take_payment() {}
    }
}

mod back_of_house {
    fn _fix_incorrect_order() {
        _cook_order();
        super::_deliver_order();
    }

    fn _cook_order() {}

    pub struct _Breakfast {
        pub toast: String,
        _seasonal_fruits: String,
    }

    impl _Breakfast {
        //This function is needed no matter what, because we have a
        //private field in the Breakfast struct. We aren't allowed to
        //call it from anywhere else, which means that without this
        //function, no Breakfast struct would ever be created.
        pub fn summer(toast: &str) -> _Breakfast {
            _Breakfast {
                toast: String::from(toast),
                _seasonal_fruits: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // //Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // //Relative path
    // front_of_house::hosting::add_to_waitlist();

    //Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::_Breakfast::summer("Rye");
    //Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    //The next line won't compile if we uncomment it; we're not allowed
    //to see or modify the seasonal fruit that comes with the meal:
    //meal.seasonal_fruit = String::from("blueberries");

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;

    hosting::_add_to_waitlist();
}
