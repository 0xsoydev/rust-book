use control_scope_modules::back_of_house::Breakfast;

fn main() {
    eat_at_restaurant();
}

pub fn eat_at_restaurant() {
    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I would like {} toast please!", meal.toast);
}
