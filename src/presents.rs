use crate::space::Space;
use std::fmt;

type PresentGrid = [[Space; 3]; 3];

macro_rules! present_without_whitespace {
    ($var:ident, $s1:expr, $s2:expr, $s3:expr) => {
        let pres_str = {
            let l1 = $s1.trim();
            let l2 = $s2.trim();
            let l3 = $s3.trim();
            format!("{}\n{}\n{}", l1, l2, l3)
        };
        #[allow(unused_mut)]
        let mut $var = Present::new(pres_str.as_str());
    };
}

mod possibilities;
pub use possibilities::PresentPossibilities;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Present {
    spaces: [[Space; 3]; 3],
}

impl Present {
    fn new(input: &str) -> Self {
        let mut spaces: PresentGrid = [[Space::Free; 3]; 3];

        for (line_num, line) in input.lines().enumerate() {
            for (char_num, char) in line.chars().enumerate() {
                spaces[line_num][char_num] = Space::new(char);
            }
        }

        Self { spaces }
    }

    fn rotate(&mut self) {
        let mut rotated_clone = self.clone();

        rotated_clone.spaces[0][0] = self.spaces[2][0];
        rotated_clone.spaces[0][1] = self.spaces[1][0];
        rotated_clone.spaces[0][2] = self.spaces[0][0];
        rotated_clone.spaces[1][0] = self.spaces[2][1];
        rotated_clone.spaces[1][2] = self.spaces[0][1];
        rotated_clone.spaces[2][0] = self.spaces[2][2];
        rotated_clone.spaces[2][1] = self.spaces[1][2];
        rotated_clone.spaces[2][2] = self.spaces[0][2];

        self.spaces = rotated_clone.spaces;
    }

    fn flip(&mut self) {
        let mut rotated_clone = self.clone();

        rotated_clone.spaces[0][0] = self.spaces[0][2];
        rotated_clone.spaces[0][2] = self.spaces[0][0];
        rotated_clone.spaces[1][0] = self.spaces[1][2];
        rotated_clone.spaces[1][2] = self.spaces[1][0];
        rotated_clone.spaces[2][0] = self.spaces[2][2];
        rotated_clone.spaces[2][2] = self.spaces[2][0];

        self.spaces = rotated_clone.spaces;
    }

    fn determine_free_space(&self) -> usize {
        let mut free_space = 0;
        for row in self.spaces {
            for space in row {
                if space == Space::Free {
                    free_space += 1;
                }
            }
        }
        free_space
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
        present_without_whitespace!(my_pres, "#.#", "..#", "##.");
        println!("{}", my_pres);
        println!();
        dbg!(my_pres);
        // panic!();
    }

    #[test]
    fn rotate() {
        present_without_whitespace!(pres, "##.", "#..", ".#.");
        present_without_whitespace!(rotated, ".##", "#.#", "...");

        pres.rotate();

        assert_eq!(pres, rotated);
    }

    #[test]
    fn rotate2() {
        present_without_whitespace!(
            pres,  //
            "###", //
            ".#.", //
            ".#."  //
        );
        present_without_whitespace!(
            rotated, //
            "..#",   //
            "###",   //
            "..#"    //
        );

        pres.rotate();

        assert_eq!(pres, rotated);
    }

    #[test]
    fn flip() {
        present_without_whitespace!(pres, "##.", "#..", ".#.");
        present_without_whitespace!(flipped, ".##", "..#", ".#.");

        pres.flip();

        assert_eq!(pres, flipped);
    }

    #[test]
    fn determine_free_space() {
        present_without_whitespace!(pres, "##.", "#..", ".#.");

        assert_eq!(5, pres.determine_free_space());
    }
}
