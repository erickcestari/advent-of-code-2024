mod day_1 {
    pub mod part_1;
    pub mod part_2;
}

mod day_2 {
  pub mod part_1;
  pub mod part_2;
}


use std::io::{self};

fn main() -> io::Result<()> {
  day_2::part_2::run()
}
