use crate::{presents::PresentPossibilities, space::Space};

pub struct Tree<'a> {
    space: Vec<Vec<Space>>,
    present_types: &'a Vec<PresentPossibilities>,
    demand: Vec<usize>,
    state: Vec<usize>,
}

impl<'a> Tree<'a> {
    pub fn new(input: &str, present_types: &'a Vec<PresentPossibilities>) -> Self {
        let (size_string, demand_string) = input.split_once(": ").unwrap();

        // Create the space vector so it is wider than it is tall
        let (dim1, dim2) = size_string.split_once('x').unwrap();
        let dim1: usize = dim1.parse().unwrap();
        let dim2: usize = dim2.parse().unwrap();
        let space: Vec<Vec<Space>> = if dim1 > dim2 {
            vec![vec![Space::Free; dim1]; dim2]
        } else {
            vec![vec![Space::Free; dim2]; dim1]
        };

        let demand: Vec<usize> = demand_string
            .trim()
            .split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        Self {
            space,
            present_types,
            demand,
            state: vec![],
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse() {
        let poss = PresentPossibilities::new("###\n###\n###");
        let poss_vec = vec![poss];
        let tree = Tree::new("21x11: 1 3 5 88", &poss_vec);

        assert_eq!(tree.space.len(), 11);
        assert_eq!(tree.space[0].len(), 21);
        assert_eq!(tree.demand.len(), 4);
        assert_eq!(tree.demand[0], 1);
        assert_eq!(tree.demand[1], 3);
        assert_eq!(tree.demand[2], 5);
        assert_eq!(tree.demand[3], 88);
    }
}
