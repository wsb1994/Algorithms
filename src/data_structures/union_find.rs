//! UnionFind/Disjoint Set data structure implementation. This code was inspired by the union find
//! implementation found in 'Algorithms Fourth Edition' by Robert Sedgewick and Kevin Wayne.
//!
//! # Resources
//!
//! - [W. Fiset's video 1](https://www.youtube.com/watch?v=ibjEGG7ylHk)
//! - [W. Fiset's video 2](https://www.youtube.com/watch?v=xq3ABa-px_g&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=31)
//! - [W. Fiset's video 3](https://www.youtube.com/watch?v=CI5Fvk-dGVs&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=32)
//! - [Wikipedia](https://www.wikiwand.com/en/Prim%27s_algorithm)

use std::cmp::Ordering::*;

/// Vector-based union-find representing a set of disjoint sets.
#[derive(Clone)]
pub struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl UnionFind {
    pub fn with_size(size: usize) -> Self {
        UnionFind {
            // parents are initialised to invalid values
            parents: (0..size).collect(),
            ranks: vec![0; size],
        }
    }

    pub fn with_ranks(ranks: Vec<usize>) -> Self {
        let size = ranks.len();
        UnionFind {
            // parents are initialised to invalid values
            parents: (0..size).collect(),
            ranks,
        }
    }

    pub fn len(&self) -> usize {
        self.parents.len()
    }

    pub fn is_empty(&self) -> bool {
        self.parents.is_empty()
    }

    pub fn extend(&mut self, size: usize) {
        let n = self.len();
        for i in n..n + size {
            self.parents.push(i);
            self.ranks.push(0);
        }
    }

    /// Try to union two sets.
    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let rep_a = self.find(a);
        let rep_b = self.find(b);

        if rep_a == rep_b {
            return false;
        }

        let rank_a = self.ranks[rep_a];
        let rank_b = self.ranks[rep_b];

        match rank_a.cmp(&rank_b) {
            Greater => self.set_parent(rep_b, rep_a),
            Less => self.set_parent(rep_a, rep_b),
            Equal => {
                self.set_parent(rep_a, rep_b);
                self.increment_rank(rep_b);
            }
        }

        true
    }

    /// Finds the representative element for the given element’s set.
    pub fn find(&mut self, mut element: usize) -> usize {
        let mut parent = self.parent(element);
        while element != parent {
            let next_parent = self.parent(parent);
            self.set_parent(element, next_parent);
            element = parent;
            parent = next_parent;
        }

        element
    }

    pub fn in_same_set(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn increment_rank(&mut self, element: usize) {
        self.ranks[element] = self.ranks[element].saturating_add(1);
    }

    pub fn parent(&self, element: usize) -> usize {
        self.parents[element]
    }

    pub fn set_parent(&mut self, element: usize, parent: usize) {
        self.parents[element] = parent;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::with_size(7);
        uf.extend(1);
        assert_eq!(uf.len(), 8);
        assert!(!uf.is_empty());
        assert!(uf.union(0, 1));
        assert!(uf.union(1, 2));
        assert!(uf.union(4, 3));
        assert!(uf.union(3, 2));
        assert!(!uf.union(0, 3));

        assert!(uf.in_same_set(0, 1));
        assert!(uf.in_same_set(0, 2));
        assert!(uf.in_same_set(0, 3));
        assert!(uf.in_same_set(0, 4));
        assert!(!uf.in_same_set(0, 5));

        uf.union(5, 3);
        assert!(uf.in_same_set(0, 5));

        uf.union(6, 7);
        assert!(uf.in_same_set(6, 7));
        assert!(!uf.in_same_set(5, 7));

        uf.union(0, 7);
        assert!(uf.in_same_set(5, 7));
    }
}
