use i_float::point::IntPoint;

pub type IntPath = Vec<IntPoint>;

pub trait PointPathExtension {
    fn unsafe_area(&self) -> i64;
    fn is_convex(&self) -> bool;
    fn is_clockwise_ordered(&self) -> bool;
    fn contains(&self, point: IntPoint) -> bool;
    fn remove_degenerates(&mut self);
    fn removed_degenerates(&self) -> IntPath;
    fn into_reversed(self) -> IntPath;
}

impl PointPathExtension for IntPath {

    /// The area of the `Path`.
    /// - Returns: A positive double area if path is clockwise and negative double area otherwise.
    fn unsafe_area(&self) -> i64 {
        let n = self.len();
        let mut p0 = self[n - 1];
        let mut area: i64 = 0;

        for &p1 in self.iter() {
            let a = (p1.x as i64).wrapping_mul(p0.y as i64);
            let b = (p1.y as i64).wrapping_mul(p0.x as i64);
            area = area.wrapping_add(a).wrapping_sub(b);
            p0 = p1;
        }

        area
    }

    /// Determines if the `Path` is convex.
    ///
    /// A convex polygon is a simple polygon (not self-intersecting) in which
    /// the line segment between any two points along the boundary never
    /// goes outside the polygon. This method assumes that the points in `Path`
    /// are ordered (either clockwise or counter-clockwise) and the path is not
    /// self-intersecting.
    ///
    /// - Returns: A Boolean value indicating whether the path is convex.
    ///   - Returns `true` if the path is convex.
    ///   - Returns `false` otherwise.
    fn is_convex(&self) -> bool {
        let n = self.len();
        if n <= 2 {
            return true;
        }

        let p0 = self[n - 2];
        let mut p1 = self[n - 1];
        let mut e0 = p1.subtract(p0);

        let mut sign: i64 = 0;
        for &p2 in self.iter() {
            let e1 = p2.subtract(p1);
            let cross = e1.cross_product(e0).signum();
            if cross == 0 {
                let dot = e1.dot_product(e0);
                if dot == -1 {
                    return false;
                }
            } else {
                if sign == 0 {
                    sign = cross
                } else if sign != cross {
                    return false;
                }
            }

            e0 = e1;
            p1 = p2;
        }

        true
    }

    /// The wind direction of the `Path`.
    /// - Returns: A Boolean value indicating whether the path is clockwise ordered.
    ///  - Returns `true` if the path is clockwise ordered.
    ///  - Returns `false` otherwise.
    #[inline(always)]
    fn is_clockwise_ordered(&self) -> bool {
        self.unsafe_area() >= 0
    }

    /// Checks if a point is contained within the `Path`.
    /// - Parameter p: The `IntPoint` point to check.
    /// - Returns: A boolean value indicating whether the point is within the path.
    fn contains(&self, point: IntPoint) -> bool {
        let n = self.len();
        let mut is_contain = false;
        let mut b = self[n - 1];
        for &a in self.iter() {
            let is_in_range = (a.y > point.y) != (b.y > point.y);
            if is_in_range {
                let dx = b.x - a.x;
                let dy = b.y - a.y;
                let sx = (point.y - a.y) * dx / dy + a.x;
                if point.x < sx {
                    is_contain = !is_contain;
                }
            }
            b = a;
        }

        is_contain
    }

    /// Removes any degenerate points from the `Path`.
    /// Degenerate points are those that are collinear with their adjacent points.
    /// After removal, the path must contain at least three non-degenerate points, or it will be cleared.
    fn remove_degenerates(&mut self) {
        if self.len() < 3 {
            self.clear();
            return;
        }

        if !has_degenerates(&self) {
            return;
        }

        let clean = filter(&self);

        self.splice(.., clean);
    }

    /// Creates a new path by removing degenerate points from the current `Path`.
    /// Similar to `remove_degenerates`, but returns a new path rather than mutating the current one.
    /// - Returns: A new `IntPoint` array with degenerates removed, or an empty array if there are fewer than three non-degenerate points.
    #[inline]
    fn removed_degenerates(&self) -> IntPath {
        if self.len() < 3 {
            return vec![IntPoint::ZERO; 0];
        }

        if !has_degenerates(&self) {
            return self.clone();
        }

        filter(&self)
    }

    #[inline]
    fn into_reversed(self) -> IntPath {
        let mut rev_path = self;
        rev_path.reverse();
        rev_path
    }
}

fn has_degenerates(path: &IntPath) -> bool {
    let count = path.len();
    let mut p0 = path[count - 2];
    let p1 = path[count - 1];

    let mut v0 = p1.subtract(p0);
    p0 = p1;

    for &pi in path.iter() {
        let vi = pi.subtract(p0);
        let prod = vi.cross_product(v0);
        if prod == 0 {
            return true;
        }
        v0 = vi;
        p0 = pi;
    }

    return false;
}

fn filter(path: &IntPath) -> IntPath {
    let mut n = path.len();
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

        let p0 = path[node.prev];
        let p1 = path[node.index];
        let p2 = path[node.next];

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
    for j in 0..n {
        buffer[j] = path[node.index];
        node = nodes[node.next];
    }

    return buffer;
}

#[derive(Clone, Copy)]
struct Node {
    next: usize,
    index: usize,
    prev: usize,
}