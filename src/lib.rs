pub mod file_parser;
use crate::{file_parser::FileParser, presents::PresentPossibilities, tree::Tree};

pub mod presents;
pub mod space;
pub mod tree;

pub fn solve_pt1(input_file: &str) -> u64 {
    let mut sections: Vec<&str> = input_file.split("\n\n").collect();

    let tree_descriptions = sections.pop().unwrap();

    let mut presents: Vec<PresentPossibilities> = Vec::with_capacity(sections.len() - 1);
    for section in sections {
        presents.push(PresentPossibilities::new(&section[3..]));
    }

    let mut trees: Vec<Tree> = Vec::with_capacity(tree_descriptions.lines().count());
    for tree_description in tree_descriptions.lines() {
        trees.push(Tree::new(tree_description, &presents));
    }

    let mut counter = 0;
    // for tree in trees {
    //     if tree.try_to_fit() {
    //         counter += 1;
    //     }
    // }
    trees[0].try_to_fit();

    counter
}

pub fn solve_pt2(input_file: &str) -> u64 {
    0
}

pub fn solve(input_file: &str) -> (u64, u64) {
    (solve_pt1(input_file), solve_pt2(input_file))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PT1: u64 = 2;
    const EXAMPLE_PT2: u64 = 0;
    const ACTUAL_PT1: u64 = 0;
    const ACTUAL_PT2: u64 = 0;
    //
    // #[test]
    // fn example() {
    //     let my_file = FileParser::new("data/example.txt");
    //     let (part_1, part_2) = solve(my_file.get_str());
    //     assert_eq!(part_1, EXAMPLE_PT1);
    //     assert_eq!(part_2, EXAMPLE_PT2);
    // }

    #[test]
    fn example_pts() {
        let my_file = FileParser::new("data/example.txt");
        assert_eq!(solve_pt1(my_file.get_str()), EXAMPLE_PT1);
        assert_eq!(solve_pt2(my_file.get_str()), EXAMPLE_PT2);
    }

    // #[test]
    // fn actual() {
    //     let my_file = FileParser::new("data/input.txt");
    //     let (part_1, part_2) = solve(my_file.get_str());
    //     assert_eq!(part_1, ACTUAL_PT1);
    //     assert_eq!(part_2, ACTUAL_PT2);
    // }
    //
    // #[test]
    // fn actual_pts() {
    //     let my_file = FileParser::new("data/input.txt");
    //     assert_eq!(solve_pt1(my_file.get_str()), ACTUAL_PT1);
    //     assert_eq!(solve_pt2(my_file.get_str()), ACTUAL_PT2);
    // }
}
