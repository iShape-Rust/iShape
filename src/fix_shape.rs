use serde::{Serialize, Deserialize};
use crate::fix_path::{FixPath, FixPathExtension };

/// Represents a fixed geometric shape with contour and holes.
#[derive(Debug, Clone,PartialEq, Eq, Deserialize, Serialize)]
pub struct FixShape {
    pub paths: Vec<FixPath>
}

impl FixShape {

    /// Is shape represent convex polygon
    pub fn is_convex_polygon(&self) -> bool {
        self.paths.len() == 1 && self.contour().is_convex()
    }

    /// Returns the contour defining the outer boundary of the shape.
    pub fn contour(&self) -> &FixPath {
        &self.paths[0]
    }

    /// Returns the array of holes defining the inner boundaries of the shape.
    pub fn holes(&self) -> &[FixPath] {
        &self.paths[1..]
    }

    /// Initializes a new shape with the specified contour.
    pub fn new_with_contour(contour: FixPath) -> Self {
        let is_contour_clockwise = contour.is_clockwise_ordered();
        let mut paths = [contour].to_vec();
        if !is_contour_clockwise {
            paths[0].reverse()
        }

        Self { paths }
    }

    /// Initializes a new shape with the specified contour and holes.
    pub fn new_with_contour_and_holes(contour: FixPath, holes: Vec<FixPath>) -> Self {
        let mut paths = Vec::with_capacity(holes.len() + 1);
        let is_contour_clockwise = contour.is_clockwise_ordered();
        paths.push(contour);
        if !is_contour_clockwise {
            paths[0].reverse()
        }

        for hole in holes.into_iter() {
            let is_clockwise = hole.is_clockwise_ordered();
            paths.push(hole);
            if is_clockwise {
                paths.last_mut().unwrap().reverse()
            }
        }

        Self { paths }
    }

    /// Initializes a new shape with the specified paths.
    /// The first path is used as the contour, and remaining paths as holes.
    pub fn new(paths: Vec<FixPath>) -> Self {
        Self { paths }
    }

    /// Adds a new hole to the shape.
    pub fn add_hole(&mut self, path: FixPath) {
        let is_clockwise = path.is_clockwise_ordered();
        self.paths.push(path);
        if is_clockwise {
            self.paths.last_mut().unwrap().reverse()
        }
    }
}