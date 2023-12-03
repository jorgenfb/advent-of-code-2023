pub mod day03 {
    pub struct Symbol {
        pub x: usize,
        pub y: usize,
    }

    pub struct Number {
        pub x: usize,
        pub y: usize,
        pub len: usize,
        pub value: u32,
    }

    impl Number {
        pub fn is_close_by(&self, symbol: &Symbol) -> bool {
            let min_x = if self.x > 0 { self.x - 1 } else { 0 };
            let min_y = if self.y > 0 { self.y - 1 } else { 0 };

            symbol.x >= min_x
                && symbol.x <= self.x + 1
                && symbol.y >= min_y
                && symbol.y <= self.y + self.len
        }
    }

    pub fn parse(input: &str) -> (Vec<Symbol>, Vec<Number>) {
        let mut symbols: Vec<Symbol> = Vec::new();
        let mut numbers: Vec<Number> = Vec::new();

        let line_len = input.lines().next().unwrap().len();

        for (x, line) in input.lines().enumerate() {
            let mut is_accumulating = false;
            let mut acc = String::new();

            for (y, char) in line.chars().enumerate() {
                // Handle numbers parsing
                let is_digit = char.is_digit(10);
                if is_digit {
                    is_accumulating = true;
                    acc.push(char);
                } else if is_accumulating {
                    // Parse accumulated number
                    let size = acc.len();
                    let num: u32 = acc.parse::<u32>().unwrap();
                    numbers.push(Number {
                        x: x,
                        y: y - size,
                        len: size,
                        value: num,
                    });
                    is_accumulating = false;
                    acc.clear();
                }

                // Handle symbols
                if !is_digit && char != '.' {
                    symbols.push(Symbol { x: x, y: y });
                }
            }

            // Repeating myself, but we need to handle the last number
            if is_accumulating {
                // Parse accumulated number
                let size = acc.len();
                let num = acc.parse::<u32>().unwrap();
                numbers.push(Number {
                    x: x,
                    y: line_len - size,
                    len: size,
                    value: num,
                });
            }
        }

        (symbols, numbers)
    }
}
