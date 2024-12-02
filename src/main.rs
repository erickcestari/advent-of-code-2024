mod day_1 {
    pub mod part_1;
    pub mod part_2;
}


use std::io::{self};

fn main() -> io::Result<()> {
  day_1::part_1::run()
}
