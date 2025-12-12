use std::collections::{HashMap, HashSet};

use super::*;

#[derive(Debug)]
pub struct PresentPossibilities {
    possibilities: Vec<Present>,
    pub free_space: usize,
}

impl PresentPossibilities {
    pub fn new(input: &str) -> Self {
        let mut possibility = Present::new(input);
        let free_space = possibility.determine_free_space();
        let mut hash_set: HashSet<Present> = HashSet::new();

        hash_set.insert(possibility.clone());

        for _ in 0..3 {
            possibility.rotate();
            hash_set.insert(possibility.clone());
        }
        possibility.flip();
        hash_set.insert(possibility.clone());
        for _ in 0..3 {
            possibility.rotate();
            hash_set.insert(possibility.clone());
        }

        Self {
            possibilities: hash_set.iter().cloned().collect(),
            free_space,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_possibilities() {
        present_without_whitespace!(pres, "#..", "...", "...");
        present_without_whitespace!(pres1, "..#", "...", "...");
        present_without_whitespace!(pres2, "...", "...", "#..");
        present_without_whitespace!(pres3, "...", "...", "..#");

        let poss = PresentPossibilities::new("#..\n...\n...");

        assert_eq!(poss.possibilities.len(), 4);
        assert_eq!(poss.free_space, 8);
        assert!(poss.possibilities.contains(&pres));
        assert!(poss.possibilities.contains(&pres1));
        assert!(poss.possibilities.contains(&pres2));
        assert!(poss.possibilities.contains(&pres3));
    }

    #[test]
    fn find_possibilities_sym() {
        present_without_whitespace!(
            pres,  //
            "###", //
            ".#.", //
            ".#."  //
        );
        present_without_whitespace!(
            pres1, //
            "..#", //
            "###", //
            "..#"  //
        );
        present_without_whitespace!(
            pres2, //
            ".#.", //
            ".#.", //
            "###"  //
        );
        present_without_whitespace!(
            pres3, //
            "#..", //
            "###", //
            "#.."  //
        );

        let poss = PresentPossibilities::new("###\n.#.\n.#.");

        assert_eq!(poss.free_space, 4);
        assert!(poss.possibilities.contains(&pres));
        assert!(poss.possibilities.contains(&pres1));
        assert!(poss.possibilities.contains(&pres2));
        assert!(poss.possibilities.contains(&pres3));
        assert_eq!(poss.possibilities.len(), 4);
    }

    #[test]
    fn find_possibilities_rot_sym() {
        present_without_whitespace!(
            pres,  //
            ".##", //
            ".#.", //
            "##."  //
        );
        present_without_whitespace!(
            pres1, //
            "#..", //
            "###", //
            "..#"  //
        );
        present_without_whitespace!(
            pres2, //
            "##.", //
            ".#.", //
            ".##"  //
        );
        present_without_whitespace!(
            pres3, //
            "..#", //
            "###", //
            "#.."  //
        );

        let poss = PresentPossibilities::new(".##\n.#.\n##.");

        assert_eq!(poss.free_space, 4);
        assert!(poss.possibilities.contains(&pres));
        assert!(poss.possibilities.contains(&pres1));
        assert!(poss.possibilities.contains(&pres2));
        assert!(poss.possibilities.contains(&pres3));
        assert_eq!(poss.possibilities.len(), 4);
    }
}
