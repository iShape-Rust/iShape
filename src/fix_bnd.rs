use i_float::fix_float::FixFloat;
use i_float::fix_vec::FixVec;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FixBnd {
    pub min: FixVec,
    pub max: FixVec,
}

impl FixBnd {

    pub const ZERO: Self = FixBnd { min: FixVec::ZERO, max: FixVec::ZERO };

    pub fn new_min_max(min: FixVec, max: FixVec) -> Self {
        Self { min, max }
    }

    pub fn new_radius(radius: FixFloat) -> Self {
        Self { min: FixVec::new_fix(-radius, -radius), max: FixVec::new_fix(radius, radius) }
    }

    pub fn center(&self) -> FixVec {
        (self.min + self.max).half()
    }

    pub fn new_two_points(p0: FixVec, p1: FixVec) -> Self {
        let xx = if p0.x < p1.x {(p0.x, p1.x)} else {(p1.x, p0.x)};
        let yy = if p0.y < p1.y {(p0.y, p1.y)} else {(p1.y, p0.y)};
        Self {
            min: FixVec::new_fix(xx.0, yy.0),
            max: FixVec::new_fix(xx.1, yy.1),
        }
    }

    pub fn union(&self, b: Self) -> Self {
        let min_x = std::cmp::min(self.min.x, b.min.x);
        let min_y = std::cmp::min(self.min.y, b.min.y);
        let max_x = std::cmp::max(self.max.x, b.max.x);
        let max_y = std::cmp::max(self.max.y, b.max.y);

        Self { min: FixVec::new_fix(min_x, min_y), max: FixVec::new_fix(max_x, max_y) }
    }

    // More methods...
}