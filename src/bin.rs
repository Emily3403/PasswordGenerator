use std::collections::HashMap;

use mylib::{count_number_of_shift_cycles, Keymap};

pub fn main() {
    let mut map = Keymap::new_emily_dvorak();

    for _ in 0..1000 {
        let pw = map.generate_small_shift_password(16);
        let num_cycles = count_number_of_shift_cycles(&pw);
        println!("{pw} {num_cycles}");
    }
}