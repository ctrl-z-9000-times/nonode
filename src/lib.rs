//! TODO copy the docs from my old python library and rewrite them for rust.

//!
//! Graph algorithms which operate directly on rust data structures.
//!
//! This library uses a novel API for representing graphs.  Graph vertexes can be
//! any hashable python value and the connectivity between vertexes is
//! represented with a callback function.  This callback is named the 'adjacent'
//! function.  The adjacent function has the following form:
//!
//! def adjacent(vertex):
//!     '''
//!     This function returns all vertexes which the given vertex is connected to.
//!     '''
//!     return iterable-of-neighboring-vertexes
//!
//! This library does not use recursion. Instead it allocates stacks on the heap.

/*
TODO: Someday I'd like to also implement:

a_star()
    Fast optimal pathfinding

topological_sort()
    Dependency resolution.

Min-cut/Max-flow
    Maybe useful?

Minimum Spanning Tree
    Maybe useful?
*/

mod depth_first;
pub use depth_first::depth_first;
pub use depth_first::depth_first_multi;
pub use depth_first::DepthFirstIter;

mod topological_sort;
pub use topological_sort::topological_sort;
pub use topological_sort::TopologicalIter;
