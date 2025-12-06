//mod front_of_house {
//    pub mod hosting {
//        pub fn add_to_waitlist() {}
//        fn seat_at_table() {}
//    }
//
//    mod serving {
//        fn take_order() {}
//        fn server_order() {}
//        fn take_payment() {}
//    }
//}
//
//pub fn eat_at_restaurant() {
//    crate::front_of_house::hosting::add_to_waitlist;
//}

pub mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    use crate::front_of_house::hosting; //This statement is necessary in order for
                                        //hosting::add_to_waitlist() to work below

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist(); //Errors out since crate initialization wasn't declared here
    }
}

//pub fn eat_at_restaurant() {
//    let mut meal = back_of_house::Breakfast::summer("Rye");
//    meal.toast = String::from("Wheat");
//    println!("I would like {} toast please!", meal.toast);
//}
