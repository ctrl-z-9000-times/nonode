use std::collections::HashSet;
use std::hash::Hash;

///
pub struct DepthFirstIter<N, F, L, R>
where
    N: Copy + Eq + Hash,
    F: Fn(N) -> L,
    L: Iterator<Item = N>,
    R: Iterator<Item = N>,
{
    ///
    pub visited: HashSet<N>,
    adj: F,
    stack: Vec<L>,
    roots: R,
}

///
pub fn depth_first<N, F, L>(root: N, adj: F) -> DepthFirstIter<N, F, L, std::option::IntoIter<N>>
where
    N: Copy + Eq + Hash,
    F: Fn(N) -> L,
    L: Iterator<Item = N>,
{
    {
        DepthFirstIter {
            visited: HashSet::new(),
            adj,
            stack: vec![],
            roots: Some(root).into_iter(),
        }
    }
}

///
pub fn depth_first_multi<N, F, L, R>(
    roots: impl IntoIterator<IntoIter = R>,
    adj: F,
) -> DepthFirstIter<N, F, L, R>
where
    N: Copy + Eq + Hash,
    F: Fn(N) -> L,
    L: Iterator<Item = N>,
    R: Iterator<Item = N>,
{
    {
        DepthFirstIter {
            visited: HashSet::new(),
            adj,
            stack: vec![],
            roots: roots.into_iter(),
        }
    }
}

impl<N, F, L, R> Iterator for DepthFirstIter<N, F, L, R>
where
    N: Copy + Eq + Hash,
    F: Fn(N) -> L,
    L: Iterator<Item = N>,
    R: Iterator<Item = N>,
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(top) = self.stack.last_mut() {
            for next in top {
                if self.visited.insert(next) {
                    self.stack.push((self.adj)(next));
                    return Some(next);
                }
            }
            self.stack.pop();
        }
        if let Some(next) = self.roots.next() {
            if self.visited.insert(next) {
                self.stack.push((self.adj)(next));
                return Some(next);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::depth_first;
    use crate::depth_first_multi;
    use std::hash::{Hash, Hasher};

    #[test]
    fn lists() {
        // Basic test, traverse a list
        let x: Vec<i32> = depth_first(0, |a| vec![a + 1].into_iter())
            .take(10)
            .collect();
        assert_eq!(x, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        // Test using references as the node type.
        let y: Vec<&i32> = depth_first(&x[0], |r| vec![&x[*r as usize + 1]].into_iter())
            .take(9)
            .collect();
        assert_eq!(y, vec![&0, &1, &2, &3, &4, &5, &6, &7, &8]);

        // Test no root nodes.
        let no_adj = |_| [].into_iter();
        let no_roots: Vec<i32> = depth_first_multi([], no_adj).collect();
        assert_eq!(no_roots, vec![]);

        // Test no leaf nodes.
        let z: Vec<i32> = depth_first_multi([0, 1, 2, 3, 4, 5], no_adj).collect();
        assert_eq!(z, vec![0, 1, 2, 3, 4, 5]);

        // Demonstrate that the API does not consume root nodes.
        let roots = [0, 1, 2, 3, 4, 5];
        assert_eq!(
            depth_first_multi(roots, no_adj).collect::<Vec<i32>>(),
            vec![0, 1, 2, 3, 4, 5]
        );
        std::mem::drop(roots);
    }

    #[test]
    fn graph() {
        let graph = vec![
            vec![0, 1, 2], // Clique of 3 nodes.
            vec![0, 1, 2],
            vec![0, 1, 2],
            vec![3, 4, 5], // Clique of 3 nodes.
            vec![3, 4, 5],
            vec![3, 4, 5],
            vec![7], // Cross ref 2 nodes together.
            vec![6],
            vec![1, 4], // Root #2.
            vec![7, 4], // Root #1.
        ];
        let adj = |idx: usize| graph[idx].iter().copied();
        let walk = depth_first_multi([9, 8], adj);
        assert_eq!(walk.collect::<Vec<_>>(), [9, 7, 6, 4, 3, 5, 8, 1, 0, 2]);
    }

    enum TreeNode {
        Leaf(i32),
        Branch(Vec<TreeNode>),
    }
    impl TreeNode {
        fn children(&self) -> std::slice::Iter<Self> {
            match self {
                TreeNode::Leaf(_) => (&[]).iter(),
                TreeNode::Branch(x) => x.iter(),
            }
        }
    }
    impl std::cmp::PartialEq for TreeNode {
        fn eq(&self, other: &TreeNode) -> bool {
            (self as *const TreeNode) == (other as *const TreeNode)
        }
    }
    impl std::cmp::Eq for TreeNode {}
    impl Hash for TreeNode {
        fn hash<H: Hasher>(&self, state: &mut H) {
            (self as *const TreeNode).hash(state)
        }
    }
    #[test]
    fn tree_inplace() {
        let t = TreeNode::Branch(vec![
            TreeNode::Branch(vec![TreeNode::Leaf(0), TreeNode::Leaf(1)]),
            TreeNode::Branch(vec![TreeNode::Leaf(2), TreeNode::Leaf(3)]),
        ]);

        let flat: Vec<_> = depth_first(&t, TreeNode::children).collect();
        assert_eq!(flat.len(), 7);
        assert!(match flat[2] {
            TreeNode::Leaf(0) => true,
            _ => false,
        });
        assert!(match flat[6] {
            TreeNode::Leaf(3) => true,
            _ => false,
        });
    }
}
