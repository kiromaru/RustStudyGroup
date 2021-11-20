pub mod hosting {
    pub fn add_to_waitlist() {
        print_message("Adding to waitlist in back of house");
    }

    pub fn seat_at_table() {
        print_message("Seating at table in back of house");
    }

    fn print_message(msg: &str) {
        println!("{}", msg);
    }
}

pub mod serving;