use crate::base::data::{Contour, Shape, Shapes};
use crate::flat::float::FloatFlatContoursBuffer;
use crate::float::adapter::{
    BufferToInt, PathToFloat, PathToInt, ShapeToFloat, ShapeToInt, ShapesToFloat, ShapesToInt,
};
use crate::int::simple::Simplify as IntSimplify;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::int::number::int::IntNumber;

/// Removes collinear and duplicate vertices using the adapter's integer coordinates.
///
/// Uses [`IntSimplify`] after conversion to integers. Collinearity and degeneracy
/// are therefore determined at the adapter's precision. Does not check or resolve
/// self-intersections, or validate winding or hole placement.
pub trait SimplifyContour<P: FloatPointCompatible, I: IntNumber> {
    /// Cleans contours in integer coordinates and converts the result back when
    /// cleanup is needed.
    ///
    /// Contours with fewer than three remaining vertices are discarded. For a
    /// shape, collapse of its first contour discards the whole shape; collapsed
    /// later contours are removed. Flat contour buffers clean each contour
    /// independently. Already empty shapes and collections are preserved.
    ///
    /// Returns `true` if any converted contour required cleanup, including an
    /// already empty contour. In that case, the surviving output is converted
    /// back through the adapter, which may also quantize otherwise unchanged
    /// contours. Returns `false` and preserves the original floating-point data
    /// if no contour required cleanup.
    fn simplify_contour(&mut self, adapter: &FloatPointAdapter<P, I>) -> bool;
}

impl<P: FloatPointCompatible, I: IntNumber> SimplifyContour<P, I> for Contour<P> {
    fn simplify_contour(&mut self, adapter: &FloatPointAdapter<P, I>) -> bool {
        let mut int_contour = self.to_int(adapter);
        if !int_contour.simplify_contour() {
            return false;
        }

        if int_contour.is_empty() {
            self.clear();
        } else {
            *self = int_contour.to_float(adapter);
        }
        true
    }
}

impl<P: FloatPointCompatible, I: IntNumber> SimplifyContour<P, I> for Shape<P> {
    fn simplify_contour(&mut self, adapter: &FloatPointAdapter<P, I>) -> bool {
        let mut int_shape = self.to_int(adapter);
        if !int_shape.simplify_contour() {
            return false;
        }

        if int_shape.is_empty() {
            self.clear();
        } else {
            *self = int_shape.to_float(adapter);
        }
        true
    }
}

impl<P: FloatPointCompatible, I: IntNumber> SimplifyContour<P, I> for Shapes<P> {
    fn simplify_contour(&mut self, adapter: &FloatPointAdapter<P, I>) -> bool {
        let mut int_shapes = self.to_int(adapter);
        if !int_shapes.simplify_contour() {
            return false;
        }

        if int_shapes.is_empty() {
            self.clear();
        } else {
            *self = int_shapes.to_float(adapter);
        }
        true
    }
}

impl<P: FloatPointCompatible, I: IntNumber> SimplifyContour<P, I> for FloatFlatContoursBuffer<P> {
    fn simplify_contour(&mut self, adapter: &FloatPointAdapter<P, I>) -> bool {
        let int_buffer = self.to_int(adapter);
        let mut output =
            FloatFlatContoursBuffer::with_capacity(int_buffer.points.len(), int_buffer.ranges.len());
        let mut changed = false;

        for mut contour in int_buffer.to_contours().into_iter() {
            changed |= contour.simplify_contour();
            if !contour.is_empty() {
                output.add_contour_iter(contour.iter().map(|p| adapter.int_to_float(p)));
            }
        }

        if changed {
            *self = output;
        }

        changed
    }
}

#[cfg(test)]
mod tests {
    use super::SimplifyContour;
    use crate::flat::float::FloatFlatContoursBuffer;
    use i_float::adapter::FloatPointAdapter;
    use i_float::float::rect::FloatRect;

    #[test]
    fn unchanged_buffer_is_not_quantized() {
        let contour = [[0.13, 0.13], [1.13, 0.13], [0.13, 1.13]];
        let mut buffer = FloatFlatContoursBuffer::default();
        buffer.add_contour(&contour);
        let original = buffer.clone();
        let adapter =
            FloatPointAdapter::<_, i32>::with_scale(FloatRect::new(0.0, 2.0, 0.0, 2.0).unwrap(), 10.0);

        let changed = buffer.simplify_contour(&adapter);

        assert!(!changed);
        assert_eq!(buffer, original);
    }
}
