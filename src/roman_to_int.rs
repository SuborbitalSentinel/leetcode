fn convert(s: String) -> i32 {
    let mut agregator = 0;
    let mut previous =  0;
    for value in s.chars().rev() {
        let current = match value {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("Not a supported roman numeral"),
        };
        if current >= previous {
            agregator += current;
        } else {
            agregator -= current;
        }
        previous = current;
    }
    return agregator;
}

#[cfg(test)]
mod tests {
    use crate::roman_to_int::convert;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let input_results = HashMap::from([
            ("I", 1),
            ("III", 3),
            ("V", 5),
            ("X", 10),
            ("XXIV", 24),
            ("L", 50),
            ("C", 100),
            ("D", 500),
            ("M", 1000),
            ("IV", 4),
            ("VI", 6),
        ]);

        for (key, value) in input_results {
            assert_eq!(convert(key.to_string()), value);
        }
    }
}
