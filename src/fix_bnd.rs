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

    pub fn new_poitns(points: &[FixVec]) -> Self {
        let p0 = points[0];
        let mut min_x = p0.x;
        let mut max_x = p0.x;
        let mut min_y = p0.y;
        let mut max_y = p0.y;

        let n = points.len();

        for i in 1..n {
            let p = points[i];

            min_x = std::cmp::min(min_x, p.x);
            min_y = std::cmp::min(min_y, p.y);
            max_x = std::cmp::max(max_x, p.x);
            max_y = std::cmp::max(max_y, p.y);
        }
        
        let min = FixVec::new_fix(min_x, min_y);
        let max = FixVec::new_fix(max_x, max_y);
        
        FixBnd { min, max }
    }

    pub fn center(self) -> FixVec {
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

    pub fn union(self, b: Self) -> Self {
        let min_x = std::cmp::min(self.min.x, b.min.x);
        let min_y = std::cmp::min(self.min.y, b.min.y);
        let max_x = std::cmp::max(self.max.x, b.max.x);
        let max_y = std::cmp::max(self.max.y, b.max.y);

        Self { min: FixVec::new_fix(min_x, min_y), max: FixVec::new_fix(max_x, max_y) }
    }

    pub fn is_collide(self, b: FixBnd) -> bool {
        // Check if the bounding boxes intersect in any dimension
        if self.max.x < b.min.x || self.min.x > b.max.x {
            return false
        }
        
        if self.max.y < b.min.y || self.min.y > b.max.y {
            return false
        }
        
        return true
    }

    pub fn is_inside(self, b: FixBnd) -> bool {
        let is_x = self.max.x >= b.max.x && self.min.x <= b.min.x;
        let is_y = self.max.y >= b.max.y && self.min.y <= b.min.y;
        
        is_x && is_y
    }

    pub fn is_contain(self, p: FixVec) -> bool {
        self.min.x <= p.x && p.x <= self.max.x && self.min.y <= p.y && p.y <= self.max.y
    }

    pub fn is_collide_circle(self, center: FixVec, radius: FixFloat) -> bool {
        let cx = std::cmp::max(self.min.x, std::cmp::min(center.x, self.max.x));
        let cy = std::cmp::max(self.min.y, std::cmp::min(center.y, self.max.y));

        let sqr_dist = FixVec::new_fix(cx, cy).sqr_distance(center);

        sqr_dist <= radius.sqr()
    }

}