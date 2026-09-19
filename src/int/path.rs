use crate::int::area::UnsafeArea;
use alloc::vec::Vec;
use i_float::int::number::int::IntNumber;
use i_float::int::number::wide_int::WideIntNumber;
use i_float::int::point::IntPoint;
use i_float::triangle::Triangle;

pub type IntPath<I> = Vec<IntPoint<I>>;
pub type IntPaths<I> = Vec<IntPath<I>>;

pub trait ContourExtension<I: IntNumber> {
    fn is_convex(&self) -> bool;
    fn is_clockwise_ordered(&self) -> bool;
    fn contains_point(&self, point: IntPoint<I>) -> bool;
}

impl<I: IntNumber> ContourExtension<I> for [IntPoint<I>] {
    /// Determines if the `Path` is convex.
    ///
    /// A convex polygon is a simple polygon (not self-intersecting) in which
    /// the line segment between any two points along the boundary never
    /// goes outside the polygon. This method assumes that the points in `Path`
    /// are ordered (either clockwise or counter-clockwise) and the path is not
    /// self-intersecting.
    ///
    /// Consecutive duplicate vertices, including repeated closing vertices, are
    /// ignored. Paths with at most two vertices after this cleanup return `true`.
    ///
    /// - Returns: A Boolean value indicating whether the path is convex.
    ///   - Returns `true` if the path is convex.
    ///   - Returns `false` otherwise.
    fn is_convex(&self) -> bool {
        let n = self.len();
        if n <= 2 {
            return true;
        }

        let mut p1 = self[n - 1];
        let Some(&p0) = self[..n - 1].iter().rev().find(|&&point| point != p1) else {
            return true;
        };
        let mut e0 = p1 - p0;

        let mut sign = I::Wide::ZERO;
        let mut edge_count = 0;
        let mut has_reversal = false;
        for &p2 in self.iter() {
            if p2 == p1 {
                continue;
            }

            edge_count += 1;
            let e1 = p2 - p1;
            let cross = e1.cross_product(e0).signum();
            if cross == I::Wide::ZERO {
                let dot = e1.dot_product(e0);
                has_reversal |= dot < I::Wide::ZERO;
            } else if sign == I::Wide::ZERO {
                sign = cross
            } else if sign != cross {
                return false;
            }

            e0 = e1;
            p1 = p2;
        }

        // A cyclic path with two distinct vertices necessarily reverses direction.
        edge_count <= 2 || !has_reversal
    }

    /// The wind direction of the `Path`.
    /// - Returns: A Boolean value indicating whether the path is clockwise ordered.
    ///  - Returns `true` if the path is clockwise ordered.
    ///  - Returns `false` otherwise.
    #[inline(always)]
    fn is_clockwise_ordered(&self) -> bool {
        self.iter().copied().unsafe_area() <= I::Wide::ZERO
    }

    /// Checks if a point is contained within the `Path`.
    /// - Parameter p: The `IntPoint` point to check.
    /// - Returns: A boolean value indicating whether the point is within the path.
    fn contains_point(&self, point: IntPoint<I>) -> bool {
        let Some(&last) = self.last() else {
            return false;
        };
        let mut is_contain = false;
        let mut b = last;
        for &a in self.iter() {
            let is_in_range = (a.y > point.y) != (b.y > point.y);
            if is_in_range {
                let is_intersection_right = if a.y < b.y {
                    Triangle::is_clockwise(a, point, b)
                } else {
                    Triangle::is_clockwise(b, point, a)
                };
                if is_intersection_right {
                    is_contain = !is_contain;
                }
            }
            b = a;
        }

        is_contain
    }
}

#[cfg(test)]
mod tests {
    use crate::int::IntPoint;
    use crate::int::area::UnsafeArea;
    use crate::int::path::ContourExtension;
    use crate::int_path;
    use alloc::vec::Vec;

    #[test]
    fn test_0() {
        let contour = int_path![
            [-314572800, 209715200],
            [-314572800, -209715200],
            [-209715200, -314572800],
            [209715200, -314572800],
            [314572800, -209715200],
            [314572800, 209715200],
            [209715200, 314572800],
            [-209715200, 314572800],
        ];

        let area: i64 = contour.into_iter().unsafe_area();
        let abs_area = area.unsigned_abs() as usize >> 1;
        assert!(area > 0);
        assert!(abs_area > 1);
    }

    #[test]
    fn empty_contour_contains_nothing() {
        let contour = Vec::<IntPoint<i32>>::new();

        assert!(!contour.contains_point(IntPoint::new(0, 0)));
    }

    #[test]
    fn convexity_detects_concavity_at_repeated_closing_vertex() {
        let mut contour = int_path![[2, 2], [0, 4], [0, 0], [4, 0], [4, 4], [2, 2]];

        assert!(!contour.is_convex());
        contour.reverse();
        assert!(!contour.is_convex());
    }

    #[test]
    fn convexity_ignores_consecutive_duplicates_in_any_position() {
        let cases = [
            (int_path![[0, 0], [4, 0], [4, 4], [0, 4]], true),
            (int_path![[0, 0], [4, 0], [4, 4], [2, 2], [0, 4]], false),
            (int_path![[0, 0], [4, 0], [2, 0], [2, 2], [0, 2]], false),
        ];

        for (mut contour, expected) in cases {
            for _ in 0..2 {
                for _ in 0..contour.len() {
                    for repeated_index in 0..contour.len() {
                        let mut repeated = contour.clone();
                        repeated.insert(repeated_index, contour[repeated_index]);
                        repeated.insert(repeated_index, contour[repeated_index]);
                        assert_eq!(repeated.is_convex(), expected, "{repeated:?}");
                        repeated.extend_from_slice(&[repeated[0]; 2]);
                        assert_eq!(repeated.is_convex(), expected, "{repeated:?}");
                    }
                    contour.rotate_left(1);
                }
                contour.reverse();
            }
        }
    }

    #[test]
    fn convexity_handles_fewer_than_three_distinct_adjacent_vertices() {
        let cases = [
            int_path![],
            int_path![[0, 0]],
            int_path![[0, 0], [1, 0]],
            int_path![[0, 0], [0, 0], [0, 0]],
            int_path![[0, 0], [0, 0], [1, 0], [1, 0]],
            int_path![[0, 0], [0, 0], [1, 0], [1, 0], [0, 0]],
        ];

        for contour in cases {
            assert!(contour.is_convex(), "{contour:?}");
        }
    }

    #[test]
    fn contains_point_uses_wide_intermediates() {
        let contour = int_path![[0, 0], [100_000, 100_000], [200_000, 0]];

        assert!(contour.contains_point(IntPoint::new(100_000, 50_000)));
        assert!(!contour.contains_point(IntPoint::new(100_000, 150_000)));
    }

    #[test]
    fn contains_point_does_not_round_edge_intersection() {
        let contour = int_path![[0, 0], [2, 0], [1, 2]];
        let mut reversed = contour.clone();
        reversed.reverse();

        assert!(!contour.contains_point(IntPoint::new(0, 1)));
        assert!(!reversed.contains_point(IntPoint::new(0, 1)));
    }
}
