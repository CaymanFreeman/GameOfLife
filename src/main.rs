mod simulation;
mod cell;
mod testing;

use crate::testing::{test_printing, test_surfaces};

fn main() {
    test_surfaces();
    test_printing();
}
