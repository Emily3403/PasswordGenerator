use std::collections::HashMap;

use rand::distributions::{Distribution, WeightedIndex};
use rand::rngs::StdRng;
use rand::SeedableRng;

pub struct Keymap {
    map: HashMap<char, f64>,
    rng: StdRng,
    final_char: char,
}

impl Keymap {
    pub fn new(map: HashMap<char, f64>) -> Self {
        return Self {
            map,
            rng: StdRng::from_entropy(),
            final_char: '.',
        };
    }

    /// Accepts a HashMap of only lowercase letters and will fill with uppercase
    pub fn new_from_lower(map: HashMap<char, f64>) -> Self {
        let mut result = map.clone();

        for (char, p) in map {
            result.insert(char.to_uppercase().next().unwrap(), p);
        }

        return Self {
            map: result,
            rng: StdRng::from_entropy(),
            final_char: '.',
        };
    }


    pub fn new_emily_dvorak() -> Self {
        let p1 = 7.0;
        let p2 = 3.0;
        let p3 = 1.0;

        Self::new_from_lower(
            HashMap::from([
                // Best candidates
                ('o', p1), ('e', p1), ('u', p1), ('i', p1),
                ('t', p1), ('n', p1), ('s', p1),

                // Second best
                ('p', p2), ('y', p2), ('q', p2), ('j', p2), ('k', p2), ('x', p2), ('a', p2), ('d', p2),
                ('c', p2), ('r', p2), ('l', p2), ('z', p2), ('v', p2), ('w', p2), ('h', p2),

                // Worst
                ('1', p3), ('2', p3), ('3', p3), ('4', p3), ('5', p3), ('6', p3), ('f', p3), ('b', p3),
                ('7', p3), ('8', p3), ('9', p3), ('0', p3), ('g', p3), ('m', p3),
            ]),
        )
    }

    pub fn generate_password(&mut self, length: usize) -> String {
        let choices = self.map.keys().cloned().collect::<Vec<char>>();
        let dist = WeightedIndex::new(self.map.values()).unwrap();

        let mut fin = String::with_capacity(length);

        for _ in 0..length - 1 {
            fin.push(choices[dist.sample(&mut self.rng)]);
        }

        fin.push(self.final_char);

        fin
    }

    pub fn generate_small_shift_password(&mut self, length: usize) -> String {
        loop {
            let pw = self.generate_password(length);

            if count_number_of_shift_cycles(&pw) <= 2 {
                return pw;
            }
        }
    }

    pub fn calculate_strength_pw(&mut self, password: &str) {
        return;
    }
}


pub fn count_number_of_shift_cycles(string: &str) -> usize {
    let mut is_up = false;
    let mut num_cycles: usize = 0;

    for it in string.chars() {
        if it.is_uppercase() != is_up {
            num_cycles += 1;
            is_up = !is_up;
        }
    }

    num_cycles.checked_sub(1).unwrap_or(0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
