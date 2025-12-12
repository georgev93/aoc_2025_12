use crate::{presents::PresentPossibilities, space::Space};

use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

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
                let x = pos_x + row_idx;
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
        let mut exploration_queue: VecDeque<(usize, usize)> = VecDeque::new();
        exploration_queue.push_front((x, y));

        while let Some(coord) = exploration_queue.pop_front() {
            match self.grid[coord.1][coord.0] {
                Space::Free => {
                    return None;
                }
                Space::Pocket => {
                    hash_set.insert(coord);
                    exploration_queue.push_back((coord.0 - 1, coord.1));
                    exploration_queue.push_back((coord.0 + 1, coord.1));
                    exploration_queue.push_back((coord.0, coord.1 - 1));
                    exploration_queue.push_back((coord.0, coord.1 + 1));
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
