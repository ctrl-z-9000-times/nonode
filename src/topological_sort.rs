use std::collections::HashSet;
use std::hash::Hash;

///
pub struct TopologicalIter<N, F, L, R>
where
    N: Copy + Eq + Hash,
    F: Fn(N) -> L,
    L: Iterator<Item = N>,
    R: Iterator<Item = N>,
{
    adj: F,
    node_stack: Vec<N>,
    adj_stack: Vec<L>,
    roots: R,
    visited: HashSet<N>,
}

///
pub fn topological_sort<N, F, L, R>(
    roots: impl IntoIterator<IntoIter = R>,
    adj: F,
) -> TopologicalIter<N, F, L, R>
where
    N: Copy + Eq + Hash,
    F: Fn(N) -> L,
    L: Iterator<Item = N>,
    R: Iterator<Item = N>,
{
    {
        TopologicalIter {
            adj,
            node_stack: vec![],
            adj_stack: vec![],
            roots: roots.into_iter(),
            visited: HashSet::new(),
        }
    }
}

impl<N, F, L, R> Iterator for TopologicalIter<N, F, L, R>
where
    N: Copy + Eq + Hash,
    F: Fn(N) -> L,
    L: Iterator<Item = N>,
    R: Iterator<Item = N>,
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        while self.adj_stack.len() == 0 {
            if let Some(next) = self.roots.next() {
                if self.visited.insert(next) {
                    self.adj_stack.push((self.adj)(next));
                    self.node_stack.push(next);
                }
            } else {
                return None;
            }
        }
        while let Some(top) = self.adj_stack.last_mut() {
            if let Some(next) = top.next() {
                if self.visited.insert(next) {
                    self.adj_stack.push((self.adj)(next));
                    self.node_stack.push(next);
                }
            } else {
                self.adj_stack.pop();
                return self.node_stack.pop();
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::topological_sort;

    #[test]
    fn graph() {
        let graph = vec![
            /* 0 */ vec![],
            /* 1 */ vec![0],
            /* 2 */ vec![1, 0],
            /* 3 */ vec![],
            /* 4 */ vec![3],
            /* 5 */ vec![3, 4],
            /* 6 */ vec![],
            /* 7 */ vec![6],
            /* 8 */ vec![2, 5],
            /* 9 */ vec![7, 5],
        ];
        let adj = |idx: usize| graph[idx].iter().copied();
        let walk = topological_sort([9, 3, 4, 5, 8, 0, 1, 2, 3, 4], adj);
        assert_eq!(walk.collect::<Vec<_>>(), [6, 7, 3, 4, 5, 9, 0, 1, 2, 8]);
    }

    #[test]
    fn empty() {
        let no_adj = |_| [].into_iter();
        // Test no root nodes.
        assert_eq!(topological_sort([], no_adj).collect::<Vec<_>>(), []);
        // Test no leaf nodes.
        assert_eq!(
            topological_sort([1, 1, 2, 2, 3, 3], no_adj).collect::<Vec<_>>(),
            [1, 2, 3]
        );
    }

    #[test]
    fn cycle() {
        // Test that cyclic graphs do not hang forever or allocate infinite memory.
        // The results are just non-sensical.
        let toggle = |b: bool| [!b].into_iter();
        assert_eq!(
            topological_sort([false], toggle).collect::<Vec<_>>(),
            [true, false]
        );
    }
}
