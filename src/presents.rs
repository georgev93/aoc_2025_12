use crate::space::Space;
use std::fmt;

type PresentGrid = [[Space; 3]; 3];

mod possibilities;
pub use possibilities::PresentPossibilities;

macro_rules! present_without_whitespace {
    ($var:ident, $s1:expr, $s2:expr, $s3:expr) => {
        let $var = {
            let l1 = $s1.trim();
            let l2 = $s2.trim();
            let l3 = $s3.trim();
            format!("{}\n{}\n{}", l1, l2, l3)
        };
    };
}

#[derive(Clone, PartialEq, Eq)]
pub struct Present {
    spaces: [[Space; 3]; 3],
    free_space: usize,
}

impl Present {
    fn new(input: &str) -> Self {
        let mut free_space = 0;
        let mut spaces: PresentGrid = [[Space::Free; 3]; 3];

        for (line_num, line) in input.lines().enumerate() {
            for (char_num, char) in line.chars().enumerate() {
                if char == '.' {
                    free_space += 1;
                }
                spaces[line_num][char_num] = Space::new(char);
            }
        }

        Self { spaces, free_space }
    }

    fn rotate(&self) -> Self {
        let mut rotated = self.clone();

        rotated.spaces[0][2] = self.spaces[0][0];
        rotated.spaces[1][2] = self.spaces[0][1];
        rotated.spaces[2][2] = self.spaces[0][2];
        rotated.spaces[2][1] = self.spaces[1][2];
        rotated.spaces[2][0] = self.spaces[2][2];
        rotated.spaces[1][0] = self.spaces[2][1];
        rotated.spaces[0][0] = self.spaces[2][0];

        rotated
    }
}

mod formatting {
    // ==================== DISPLAY SECTION BEGINS ====================
    use super::*;

    impl fmt::Display for Present {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for row in &self.spaces {
                for space in row {
                    write!(f, "{}", space)?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }

    impl fmt::Debug for Present {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f)?;
            fmt::Display::fmt(self, f)
        }
    }
    // ==================== DISPLAY SECTION ENDS ====================
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn printing() {
        present_without_whitespace!(pres_str, "#.#", "..#", "##.");
        let my_pres = Present::new(pres_str.as_str());
        println!("{}", my_pres);
        println!();
        dbg!(my_pres);
        // panic!();
    }

    #[test]
    fn rotate() {
        present_without_whitespace!(pres_str, "##.", "#..", ".#.");
        present_without_whitespace!(rotated_str, ".##", "#.#", "...");
        let mut pres = Present::new(pres_str.as_str());
        let rotated = Present::new(rotated_str.as_str());

        pres = pres.rotate();

        assert_eq!(pres, rotated);
    }
}
