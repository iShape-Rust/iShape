use alloc::vec::Vec;
use core::ops::Range;
use i_float::int::point::IntPoint;
use crate::int::shape::{IntContour, IntShape,};

pub struct FlatGeometry {
    pub(crate) points: Vec<IntPoint>,
    pub(crate) contours: Vec<Range<usize>>,
    pub(crate) shapes: Vec<Range<usize>>,
}

impl FlatGeometry {

    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            points: Vec::with_capacity(capacity),
            contours: Vec::new(),
            shapes: Vec::new(),
        }
    }

    #[inline]
    fn init_with_contour(&mut self, contour: &[IntPoint]) {
        self.points.clear();
        self.points.extend_from_slice(contour);
        self.contours.push(0..self.points.len());
        self.shapes.push(0..1);
    }

    #[inline]
    fn init_with_shape(&mut self, shape: &[IntContour]) {
        self.points.clear();
        let mut offset = 0;
        for contour in shape.iter() {
            self.points.extend_from_slice(contour);
            self.contours.push(offset..self.points.len());
            offset += self.points.len();
        }

        self.shapes.push(0..shape.len());
    }

    #[inline]
    fn init_with_shapes(&mut self, shapes: &[IntShape]) {
        self.points.clear();
        let mut points_offset = 0;
        let mut contours_offset = 0;
        for shape in shapes.iter() {
            for contour in shape.iter() {
                self.points.extend_from_slice(contour);
                self.contours.push(points_offset..self.points.len());
                points_offset += self.points.len();
            }
            
            self.shapes.push(contours_offset..self.contours.len());
            contours_offset += self.contours.len();
        }
    }
}