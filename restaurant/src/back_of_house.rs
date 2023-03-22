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
