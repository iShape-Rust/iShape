use i_float::int::point::IntPoint;
use crate::int::path::IntPath;

pub trait Simple {
    fn is_simple(&self) -> bool;
    fn to_simple(&self) -> IntPath;
}

impl Simple for [IntPoint] {
    fn is_simple(&self) -> bool {
        let count = self.len();

        if count < 3 { return false; }

        let mut p0 = self[count - 2];
        let p1 = self[count - 1];

        let mut v0 = p1.subtract(p0);
        p0 = p1;

        for &pi in self.iter() {
            let vi = pi.subtract(p0);
            let prod = vi.cross_product(v0);
            if prod == 0 {
                return false;
            }
            v0 = vi;
            p0 = pi;
        }

        true
    }

    /// Creates a new path by removing degenerate points from the current `Path`.
    /// Similar to `remove_degenerates`, but returns a new path rather than mutating the current one.
    /// - Returns: A new `IntPoint` array with degenerates removed, or an empty array if there are fewer than three non-degenerate points.
    fn to_simple(&self) -> IntPath {
        if self.len() < 3 {
            return vec![IntPoint::ZERO; 0];
        }

        let mut n = self.len();
        let mut nodes: Vec<Node> = vec![Node { next: 0, index: 0, prev: 0 }; n];
        let mut validated: Vec<bool> = vec![false; n];

        let mut i0 = n - 2;
        let mut i1 = n - 1;
        for i2 in 0..n {
            nodes[i1] = Node { next: i2, index: i1, prev: i0 };
            i0 = i1;
            i1 = i2;
        }

        let mut first: usize = 0;
        let mut node = nodes[first];
        let mut i = 0;
        while i < n {
            if validated[node.index] {
                node = nodes[node.next];
                continue;
            }

            let p0 = self[node.prev];
            let p1 = self[node.index];
            let p2 = self[node.next];

            if p1.subtract(p0).cross_product(p2.subtract(p1)) == 0 {
                n -= 1;
                if n < 3 {
                    return vec![IntPoint::ZERO; 0];
                }

                // remove node
                nodes[node.prev].next = node.next;
                nodes[node.next].prev = node.prev;

                if node.index == first {
                    first = node.next
                }

                node = nodes[node.prev];

                if validated[node.prev] {
                    i -= 1;
                    validated[node.prev] = false
                }

                if validated[node.next] {
                    i -= 1;
                    validated[node.next] = false
                }

                if validated[node.index] {
                    i -= 1;
                    validated[node.index] = false
                }
            } else {
                validated[node.index] = true;
                i += 1;
                node = nodes[node.next];
            }
        }

        let mut buffer = vec![IntPoint::ZERO; n];
        node = nodes[first];

        for item in buffer.iter_mut().take(n) {
            *item = self[node.index];
            node = nodes[node.next];
        }

        buffer
    }
}

#[derive(Clone, Copy)]
struct Node {
    next: usize,
    index: usize,
    prev: usize,
}