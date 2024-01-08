use i_float::fix_float::FixFloat;
use i_float::fix_vec::FixVec;


pub struct Triangle;

impl Triangle {
    
    pub fn unsafe_area_two(p0: FixVec, p1: FixVec, p2: FixVec) -> i64 {
        (p1 - p0).unsafe_cross_product(p1 - p2)
    }

    pub fn unsafe_area(p0: FixVec, p1: FixVec, p2: FixVec) -> i64 {
        Self::unsafe_area_two(p0, p1, p2) / 2
    }

    pub fn fix_area(p0: FixVec, p1: FixVec, p2: FixVec) -> FixFloat {
        (p1 - p0).cross_product(p1 - p2) / 2
    }

    pub fn is_clockwise(p0: FixVec, p1: FixVec, p2: FixVec) -> bool {
        Self::unsafe_area_two(p0, p1, p2) > 0
    }

    pub fn is_cw_or_line(p0: FixVec, p1: FixVec, p2: FixVec) -> bool {
        Self::unsafe_area_two(p0, p1, p2) >= 0
    }

    pub fn is_not_line(p0: FixVec, p1: FixVec, p2: FixVec) -> bool {
        Self::unsafe_area_two(p0, p1, p2) != 0
    }

    pub fn clock_direction(p0: FixVec, p1: FixVec, p2: FixVec) -> i64 {
        let area = Self::unsafe_area_two(p0, p1, p2);
        if area < 0 {
            return -1;
        }
        if area > 0 {
            return 1;
        }
        return 0;
    }

    pub fn is_contain(p: FixVec, p0: FixVec, p1: FixVec, p2: FixVec) -> bool {
        let q0 = (p - p1).unsafe_cross_product(p0 - p1);
        let q1 = (p - p2).unsafe_cross_product(p1 - p2);
        let q2 = (p - p0).unsafe_cross_product(p2 - p0);
        
        let has_neg = q0 < 0 || q1 < 0 || q2 < 0;
        let has_pos = q0 > 0 || q1 > 0 || q2 > 0;
        
        !(has_neg && has_pos)
    }

    pub fn is_not_contain(p: FixVec, p0: FixVec, p1: FixVec, p2: FixVec) -> bool {
        let q0 = (p - p1).unsafe_cross_product(p0 - p1);
        let q1 = (p - p2).unsafe_cross_product(p1 - p2);
        let q2 = (p - p0).unsafe_cross_product(p2 - p0);
        
        let has_neg = q0 <= 0 || q1 <= 0 || q2 <= 0;
        let has_pos = q0 >= 0 || q1 >= 0 || q2 >= 0;
        
        has_neg && has_pos
    }

}