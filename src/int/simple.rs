//! Removes vertices whose adjacent edges have a zero cross product.
//!
//! Contours are treated as cyclic paths. Cleanup includes consecutive duplicate
//! points, collinear vertices, and collinear edge reversals. In this module,
//! "simple" means at least three vertices with no such adjacent edges; it does
//! not mean a polygon without self-intersections. These operations neither check
//! nor resolve self-intersections, and do not validate winding or hole placement.
//! Coordinates must satisfy the arithmetic range documented by [`IntPoint`].

use crate::int::shape::{IntContour, IntShape, IntShapes};
use alloc::vec;
use alloc::vec::Vec;
use i_float::int::number::int::IntNumber;
use i_float::int::number::wide_int::WideIntNumber;
use i_float::int::point::IntPoint;

/// In-place removal of collinear and duplicate vertices from integer contours.
pub trait Simplify {
    /// Repeatedly removes vertices whose adjacent edges have a zero cross product.
    ///
    /// A contour is cleared if fewer than three vertices remain. In a shape, the
    /// first contour is treated as the outer boundary: if it collapses, the whole
    /// shape is cleared; collapsed later contours are removed. Collections of
    /// shapes discard shapes whose outer boundary collapses. Already empty shapes
    /// and collections are left unchanged.
    ///
    /// Returns `true` if any contour required cleanup, including an already empty
    /// contour, or `false` if every contour passed [`SimpleContour::is_simple`].
    /// Does not check or resolve self-intersections.
    fn simplify_contour(&mut self) -> bool;
}

/// Checks for redundant adjacent vertices and creates a cleaned contour copy.
pub trait SimpleContour<I: IntNumber> {
    /// Returns `true` if there are at least three vertices and every pair of
    /// adjacent edges has a non-zero cross product, including at the closing edge.
    ///
    /// Consecutive duplicate points and collinear triples return `false`.
    /// Self-intersections are not checked: a self-intersecting contour can return
    /// `true` if all its local turns have non-zero cross products.
    fn is_simple(&self) -> bool;

    /// Returns a copy with collinear and consecutive duplicate vertices removed.
    ///
    /// Removal repeats until all adjacent edge pairs have non-zero cross products.
    /// Returns `None` if the input has fewer than three vertices or cleanup leaves
    /// fewer than three. Does not check or resolve self-intersections.
    fn simplified(&self) -> Option<IntContour<I>>;
}

/// Checks and removes redundant adjacent vertices in a shape's contours.
pub trait SimpleShape<I: IntNumber> {
    /// Returns whether every contour passes [`SimpleContour::is_simple`].
    ///
    /// An empty shape returns `true`. Does not validate self-intersections,
    /// winding, or the relationship between the outer boundary and holes.
    fn is_simple(&self) -> bool;

    /// Returns a copy with each contour cleaned using [`SimpleContour::simplified`]
    /// when needed.
    ///
    /// The first contour is treated as the outer boundary. Returns `None` if that
    /// contour collapses; collapsed later contours are omitted. An empty input
    /// returns `Some` containing an empty shape.
    fn simplified(&self) -> Option<IntShape<I>>;
}

/// Checks and removes redundant adjacent vertices in a collection of shapes.
pub trait SimpleShapes<I: IntNumber> {
    /// Returns whether every shape passes [`SimpleShape::is_simple`].
    ///
    /// An empty collection returns `true`. Does not check self-intersections or
    /// relationships between shapes.
    fn is_simple(&self) -> bool;

    /// Returns cleaned copies of the shapes using [`SimpleShape::simplified`]
    /// when needed.
    ///
    /// Shapes whose outer boundary collapses are omitted. Already empty shapes
    /// are preserved; an empty collection returns an empty collection.
    fn simplified(&self) -> IntShapes<I>;
}

impl<I: IntNumber> Simplify for IntContour<I> {
    #[inline]
    fn simplify_contour(&mut self) -> bool {
        if self.is_simple() {
            return false;
        }
        if let Some(contour) = self.simplified() {
            self.clear();
            self.extend(contour);
        } else {
            self.clear()
        }
        true
    }
}

impl<I: IntNumber> Simplify for IntShape<I> {
    fn simplify_contour(&mut self) -> bool {
        let mut any_simplified = false;
        let mut any_empty = false;

        for (index, contour) in self.iter_mut().enumerate() {
            if contour.is_simple() {
                continue;
            }
            any_simplified = true;

            if let Some(simple_contour) = contour.simplified() {
                contour.clear();
                contour.extend(simple_contour);
            } else if index == 0 {
                // early out main contour is empty
                self.clear();
                return true;
            } else {
                contour.clear();
                any_empty = true;
            }
        }

        if any_empty {
            self.retain(|contour| !contour.is_empty());
        }

        any_simplified
    }
}

impl<I: IntNumber> Simplify for IntShapes<I> {
    fn simplify_contour(&mut self) -> bool {
        let mut any_simplified = false;
        let mut any_empty = false;

        for shape in self.iter_mut() {
            if shape.is_simple() {
                continue;
            }
            any_simplified = true;
            if let Some(simple_shape) = shape.simplified() {
                *shape = simple_shape;
            } else {
                shape.clear();
                any_empty = true;
            }
        }

        if any_empty {
            self.retain(|contour| !contour.is_empty());
        }

        any_simplified
    }
}
impl<I: IntNumber> SimpleShape<I> for [IntContour<I>] {
    #[inline]
    fn is_simple(&self) -> bool {
        for contour in self.iter() {
            if !contour.is_simple() {
                return false;
            }
        }
        true
    }

    fn simplified(&self) -> Option<IntShape<I>> {
        let mut contours = Vec::with_capacity(self.len());
        for (i, contour) in self.iter().enumerate() {
            if contour.is_simple() {
                contours.push(contour.clone());
            } else if let Some(simple) = contour.simplified() {
                contours.push(simple);
            } else if i == 0 {
                return None;
            }
        }

        Some(contours)
    }
}

impl<I: IntNumber> SimpleShapes<I> for [IntShape<I>] {
    #[inline]
    fn is_simple(&self) -> bool {
        for shape in self.iter() {
            if !shape.is_simple() {
                return false;
            }
        }
        true
    }

    fn simplified(&self) -> IntShapes<I> {
        let mut shapes = Vec::with_capacity(self.len());
        for shape in self.iter() {
            if shape.is_simple() {
                shapes.push(shape.clone());
            } else if let Some(simple) = shape.simplified() {
                shapes.push(simple);
            }
        }

        shapes
    }
}

impl<I: IntNumber> SimpleContour<I> for [IntPoint<I>] {
    fn is_simple(&self) -> bool {
        let count = self.len();

        if count < 3 {
            return false;
        }

        let mut p0 = self[count - 2];
        let p1 = self[count - 1];

        let mut v0 = p1 - p0;
        p0 = p1;

        for &pi in self.iter() {
            let vi = pi - p0;
            let prod = vi.cross_product(v0);
            if prod == I::Wide::ZERO {
                return false;
            }
            v0 = vi;
            p0 = pi;
        }

        true
    }

    #[inline]
    fn simplified(&self) -> Option<IntContour<I>> {
        ContourSimplifier::default().simplify_contour(self)
    }
}

/// Reusable scratch storage for removing collinear and duplicate vertices.
#[derive(Default)]
pub struct ContourSimplifier {
    nodes: Vec<Node>,
    validated: Vec<bool>,
}

impl ContourSimplifier {
    /// Returns a cleaned contour copy with the same behavior as
    /// [`SimpleContour::simplified`], reusing internal scratch allocations.
    pub fn simplify_contour<I: IntNumber>(&mut self, contour: &[IntPoint<I>]) -> Option<IntContour<I>> {
        let mut n = contour.len();

        if n < 3 {
            return None;
        }

        self.validated.clear();
        self.validated.resize(n, false);

        self.nodes.clear();
        self.nodes.reserve(n);

        let mut prev = n - 1;
        let mut next = 1;
        let last = n - 1;
        #[allow(clippy::explicit_counter_loop)]
        for index in 0..last {
            self.nodes.push(Node { next, index, prev });
            prev = index;
            next += 1;
        }
        self.nodes.push(Node {
            next: 0,
            index: last,
            prev,
        });

        let mut first: usize = 0;
        let mut node = self.nodes[first];
        let mut i = 0;
        while i < n {
            if self.validated[node.index] {
                node = self.nodes[node.next];
                continue;
            }

            let p0 = contour[node.prev];
            let p1 = contour[node.index];
            let p2 = contour[node.next];

            if (p1 - p0).cross_product(p2 - p1) == I::Wide::ZERO {
                n -= 1;
                if n < 3 {
                    return None;
                }

                // remove node
                self.nodes[node.prev].next = node.next;
                self.nodes[node.next].prev = node.prev;

                if node.index == first {
                    first = node.next
                }

                node = self.nodes[node.prev];

                if self.validated[node.prev] {
                    i -= 1;
                    self.validated[node.prev] = false
                }

                if self.validated[node.next] {
                    i -= 1;
                    self.validated[node.next] = false
                }

                if self.validated[node.index] {
                    i -= 1;
                    self.validated[node.index] = false
                }
            } else {
                self.validated[node.index] = true;
                i += 1;
                node = self.nodes[node.next];
            }
        }

        let mut buffer = vec![IntPoint::<I>::ZERO; n];
        node = self.nodes[first];

        for item in buffer.iter_mut().take(n) {
            *item = contour[node.index];
            node = self.nodes[node.next];
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
