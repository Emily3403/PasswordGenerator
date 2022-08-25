use mylib::{count_number_of_shift_cycles, Keymap};

pub fn main() {
    let mut map = Keymap::new_emily_dvorak();

    for _ in 0..10 {
        let pw = map.generate_password(20);
        let num_cycles = count_number_of_shift_cycles(&pw);
        println!("{pw} {num_cycles}");
    }
}