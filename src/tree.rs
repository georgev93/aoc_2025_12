use crate::{presents::PresentPossibilities, space::Space};

use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

// Present_idx, Possibility_idx
type PossibleMove = (usize, usize);

#[derive(Debug)]
pub struct Tree<'a> {
    grid: Vec<Vec<Space>>,
    present_types: &'a Vec<PresentPossibilities>,
    demand: Vec<usize>,
    state: Vec<usize>,
    space_slack: isize,
}

impl<'a> Tree<'a> {
    pub fn new(input: &str, present_types: &'a Vec<PresentPossibilities>) -> Self {
        let (size_string, demand_string) = input.split_once(": ").unwrap();

        // Create the space vector so it is wider than it is tall
        let (height, width) = size_string.split_once('x').unwrap();
        let mut height: usize = height.parse::<usize>().unwrap() + 2;
        let mut width: usize = width.parse::<usize>().unwrap() + 2;
        if height > width {
            // The 'ol switcheroo
            std::mem::swap(&mut width, &mut height);
        }

        let mut grid = vec![vec![Space::Free; width]; height];

        for col in 0..width {
            grid[0][col] = Space::Occupied;
            grid[height - 1][col] = Space::Occupied;
        }
        for row in 0..height {
            grid[row][0] = Space::Occupied;
            grid[row][width - 1] = Space::Occupied;
        }

        let demand: Vec<usize> = demand_string
            .trim()
            .split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let mut free_space_remaining = (height - 2) * (width - 2);
        for (present_idx, present) in present_types.iter().enumerate() {
            free_space_remaining =
                free_space_remaining.saturating_sub(present.get_size() * demand[present_idx]);
        }

        Self {
            grid,
            present_types,
            demand,
            space_slack: free_space_remaining as isize,
            state: vec![],
        }
    }

    pub fn clone_from_self(&self) -> Self {
        Self {
            grid: self.grid.clone(),
            present_types: self.present_types,
            demand: self.demand.clone(),
            state: self.state.clone(),
            space_slack: self.space_slack,
        }
    }

    pub fn try_to_fit(&self) -> bool {
        if self.space_slack < 0 {
            return false;
        }

        if self.demand.iter().all(|&x| x == 0) {
            return true;
        }

        let mut possible_move_vector: Vec<PossibleMove> = Vec::new();
        for (present_idx, present) in self.present_types.iter().enumerate() {
            if self.demand[present_idx] == 0 {
                continue;
            }
            for possibility_index in 0..self.present_types[present_idx].possibilities.len() {
                possible_move_vector.push((present_idx, possibility_index));
            }
        }

        let mut possible_trees: Vec<Tree> = Vec::new();
        for row in 2..(self.grid.len() - 2) {
            for col in 2..(self.grid[0].len() - 2) {
                for possible_move in &possible_move_vector {
                    let mut possible_tree = self.clone_from_self();
                    if possible_tree.place_present(possible_move.0, possible_move.1, col, row) {
                        possible_trees.push(possible_tree);
                    }
                }
            }
        }

        // println!("GOT HERE");

        for tree in possible_trees {
            println!("Trying tree: ");
            if tree.try_to_fit() {
                return true;
            }
        }
        false
    }

    pub fn place_present(
        &mut self,
        present_idx: usize,
        poss_idx: usize,
        pos_x: usize,
        pos_y: usize,
    ) -> bool {
        let pres_ref = &self.present_types[present_idx].possibilities[poss_idx];

        // Starting at the top left
        let pos_x = pos_x - 1;
        let pos_y = pos_y - 1;

        let mut pocket_vec: Vec<(usize, usize)> = Vec::new();

        for (row_idx, row) in pres_ref.spaces.iter().enumerate() {
            for (space_idx, pres_space) in row.iter().enumerate() {
                let x = pos_x + space_idx;
                let y = pos_y + row_idx;

                let current_space = &mut self.grid[y][x];

                if *pres_space == Space::Occupied {
                    if *current_space == Space::Occupied {
                        // COLLISION
                        return false;
                    } else {
                        // NEW OCCUPATION
                        *current_space = Space::Occupied;
                    }
                } else if *current_space != Space::Occupied {
                    // IT HAS POCKETSSS
                    *current_space = Space::Pocket;
                    pocket_vec.push((x, y));
                }
            }
        }

        let mut known_complete_pocket_coords: HashSet<(usize, usize)> = HashSet::new();
        while let Some(pocket_coord) = pocket_vec.pop() {
            if known_complete_pocket_coords.contains(&pocket_coord) {
                continue;
            }
            if let Some(pocket_coords_hash) = self.explore_pocket(pocket_coord.0, pocket_coord.1) {
                known_complete_pocket_coords.extend(pocket_coords_hash);
            }
        }

        for coord in &known_complete_pocket_coords {
            self.grid[coord.1][coord.0] = Space::Occupied;
        }

        self.space_slack -= known_complete_pocket_coords.len() as isize;

        self.demand[present_idx] -= 1;
        true
    }

    fn explore_pocket(&self, x: usize, y: usize) -> Option<HashSet<(usize, usize)>> {
        let mut hash_set: HashSet<(usize, usize)> = HashSet::new();
        let mut to_examine: HashSet<(usize, usize)> = HashSet::new();
        let mut exploration_queue: VecDeque<(usize, usize)> = VecDeque::new();

        to_examine.insert((x, y));
        exploration_queue.push_front((x, y));

        while let Some(coord) = exploration_queue.pop_front() {
            match self.grid[coord.1][coord.0] {
                Space::Free => {
                    return None;
                }
                Space::Pocket => {
                    hash_set.insert(coord);
                    if to_examine.insert((coord.0 - 1, coord.1)) {
                        exploration_queue.push_back((coord.0 - 1, coord.1));
                    }
                    if to_examine.insert((coord.0 + 1, coord.1)) {
                        exploration_queue.push_back((coord.0 + 1, coord.1));
                    }
                    if to_examine.insert((coord.0, coord.1 - 1)) {
                        exploration_queue.push_back((coord.0, coord.1 - 1));
                    }
                    if to_examine.insert((coord.0, coord.1 + 1)) {
                        exploration_queue.push_back((coord.0, coord.1 + 1));
                    }
                }
                Space::Occupied => {}
            }
        }
        Some(hash_set)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse() {
        let poss = PresentPossibilities::new(".##\n###\n###"); // idx: 0, space: 8
        let poss1 = PresentPossibilities::new("...\n#.#\n###"); // idx: 1, space: 5
        let poss2 = PresentPossibilities::new(".##\n...\n.##"); // idx: 2, space: 4
        let poss_vec = vec![poss, poss1, poss2];
        let tree = Tree::new("21x11: 1 5 10", &poss_vec);

        assert_eq!(tree.grid.len(), 13);
        assert_eq!(tree.grid[0].len(), 23);
        assert_eq!(tree.demand.len(), 3);
        assert_eq!(tree.demand[0], 1);
        assert_eq!(tree.demand[1], 5);
        assert_eq!(tree.demand[2], 10);
        assert_eq!(tree.space_slack, 158); // (21*11) - (8*1 + 5*5 + 4*10)
    }

    #[test]
    fn place_presents() {
        let poss = PresentPossibilities::new(".##\n###\n###"); // idx: 0, space: 8
        let poss1 = PresentPossibilities::new("...\n#.#\n###"); // idx: 1, space: 5
        let poss2 = PresentPossibilities::new(".##\n...\n.##"); // idx: 2, space: 4
        let poss_vec = vec![poss, poss1, poss2];
        let tree = Tree::new("21x11: 1 5 10", &poss_vec);

        assert_eq!(tree.grid.len(), 13);
        assert_eq!(tree.grid[0].len(), 23);
        assert_eq!(tree.demand.len(), 3);
        assert_eq!(tree.demand[0], 1);
        assert_eq!(tree.demand[1], 5);
        assert_eq!(tree.demand[2], 10);
        assert_eq!(tree.space_slack, 158); // (21*11) - (8*1 + 5*5 + 4*10)
    }
}
