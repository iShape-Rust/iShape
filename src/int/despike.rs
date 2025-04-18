use i_float::int::point::IntPoint;
use crate::int::shape::IntContour;

/// A trait for removing spike artifacts from polygon contours.
pub trait DeSpike {
    /// Removes spikes from the contour in-place.
    ///
    /// # Returns
    ///
    /// - `true` if spikes were found and removed.
    /// - `false` if the contour was already clean.
    fn remove_spikes(&mut self) -> bool;

    /// Checks whether the contour has no spikes.
    ///
    /// A contour with no spikes is considered clean and valid
    /// for most geometric operations.
    ///
    /// # Returns
    ///
    /// - `true` if the contour has no spike patterns.
    /// - `false` if any spike-like edge reversal is detected.
    fn has_no_spikes(&self) -> bool;

    /// Returns a copy of the contour with spikes removed.
    ///
    /// # Returns
    ///
    /// - `Some(IntContour)` if a valid, despiked contour can be produced.
    /// - `None` if the contour is degenerate after spike removal.
    fn despiked_contour(&self) -> Option<IntContour>;
}

impl DeSpike for IntContour {
    fn remove_spikes(&mut self) -> bool {
        if self.has_no_spikes() {
            return false;
        }
        if let Some(contour) = self.despiked_contour() {
            *self = contour;
        } else {
            self.clear()
        }
        true
    }

    fn has_no_spikes(&self) -> bool {
        let count = self.len();

        if count < 3 { return false; }

        let mut p0 = self[count - 2];
        let p1 = self[count - 1];

        let mut v0 = p1.subtract(p0);
        p0 = p1;

        for &pi in self.iter() {
            let vi = pi.subtract(p0);
            let cross = vi.cross_product(v0);
            let dot = vi.dot_product(v0);
            if cross == 0 && dot < 0 {
                return false;
            }
            v0 = vi;
            p0 = pi;
        }

        true
    }

    fn despiked_contour(&self) -> Option<IntContour> {
        if self.len() < 3 {
            return None;
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

            let v10 = p1.subtract(p0);
            let v21 = p2.subtract(p1);
            let cross = v10.cross_product(v21);
            let dot = v10.dot_product(v21);

            if cross == 0 && dot < 0 {
                n -= 1;
                if n < 3 {
                    return None;
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

        Some(buffer)
    }
}

#[derive(Clone, Copy)]
struct Node {
    next: usize,
    index: usize,
    prev: usize,
}

#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use crate::int::despike::DeSpike;

    #[test]
    fn test_0() {
        let mut contour =
            vec![
                IntPoint::new(0, 0),
                IntPoint::new(1, 0),
                IntPoint::new(1, 1),
                IntPoint::new(0, 1),
            ];

        let modified = contour.remove_spikes();

        assert_eq!(contour.len(), 4);
        assert_eq!(modified, false);
    }

    #[test]
    fn test_1() {
        let mut contour =
            vec![
                IntPoint::new(0, -1),
                IntPoint::new(0, 1),
                IntPoint::new(1, 1),
                IntPoint::new(1, 0),
                IntPoint::new(0, 0),
            ];

        let modified = contour.remove_spikes();

        assert_eq!(contour.len(), 4);
        assert_eq!(modified, true);
    }

    #[test]
    fn test_2() {
        let mut contour =
            vec![
                IntPoint::new(0, -1),
                IntPoint::new(0, 1),
                IntPoint::new(1, 1),
                IntPoint::new(1, 0),
                IntPoint::new(0, 0),
            ];

        let modified = contour.remove_spikes();

        assert_eq!(contour.len(), 4);
        assert_eq!(modified, true);
    }

    #[test]
    fn test_3() {
        let mut contour =
            vec![
                IntPoint::new(0, 0),
                IntPoint::new(0, 2),
                IntPoint::new(1, 2),
                IntPoint::new(3, 2),
                IntPoint::new(4, 2),
                IntPoint::new(2, 2),
                IntPoint::new(2, 0),
            ];

        let modified = contour.remove_spikes();

        assert_eq!(contour.len(), 5);
        assert_eq!(modified, true);
    }

    #[test]
    fn test_4() {
        let mut contour =
            vec![
                IntPoint::new(0, 0),
                IntPoint::new(0, 2),
                IntPoint::new(1, 2),
                IntPoint::new(4, 2),
                IntPoint::new(3, 2),
                IntPoint::new(2, 2),
                IntPoint::new(2, 0),
            ];

        let modified = contour.remove_spikes();

        assert_eq!(contour.len(), 5);
        assert_eq!(modified, true);
    }
}