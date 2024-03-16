mod simulation;
mod cell;
mod testing;
mod seeds;

use crate::testing::{test_printing, test_surfaces};

fn main() {
    test_surfaces();
    test_printing();
}
