// Defining a module front_of_house
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            print_message("Adding to waitlist in front of house");
        }

        pub fn seat_at_table() {
            print_message("Seating at table in front of house");
        }

        fn print_message(msg: &str) {
            println!("{}", msg);
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("Taking order in front of house");
        }

        pub fn serve_order() {
            println!("Serving order in front of house");
        }

        pub fn take_payment() {
            println!("Taking payment in front of house");
        }
    }
}

// Declaring that we have such a module
mod back_of_house;

fn main() {
    crate::front_of_house::serving::take_order();
    crate::back_of_house::serving::take_order();
}
