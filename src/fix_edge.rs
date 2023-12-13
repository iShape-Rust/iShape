use i_float::fix_vec::FixVec;
use crate::triangle::Triangle;
use crate::fix_bnd::FixBnd;

#[derive(Debug, Clone, Copy)]
pub struct EdgeCross {
    pub nature: EdgeCrossType,
    pub point: FixVec,
    pub second: FixVec,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EdgeCrossType {
    NotCross, 
    Pure,
    OverlayA, 
    OverlayB, 
    Penetrate, 
    EndA, 
    EndB,
}

#[derive(Debug, Clone, Copy)]
pub struct FixEdge {
    pub e0: FixVec,
    pub e1: FixVec,
}

impl FixEdge {

    pub const ZERO: FixEdge = FixEdge {
        e0: FixVec::ZERO,
        e1: FixVec::ZERO,
    };
    
    pub fn new(e0: FixVec, e1: FixVec) -> Self {
        Self { e0, e1 }
    }

    pub fn cross(&self, other: FixEdge) -> Option<EdgeCross> {
        let a0 = self.e0;
        let a1 = self.e1;

        let b0 = other.e0;
        let b1 = other.e1;

        let a0_area = Triangle::unsafe_area_two(b0, a0, b1);
        let a1_area = Triangle::unsafe_area_two(b0, a1, b1);
        
        if a0_area == 0 && a1_area == 0 {
            // same line
            return FixEdge::same_line_overlay(*self, other);
        }

        let com_a0 = a0 == b0 || a0 == b1;
        let com_a1 = a1 == b0 || a1 == b1;

        let has_same_end = com_a0 || com_a1;

        if has_same_end {
            return None;
        }

        if a0_area == 0 {
            if other.is_box_contain(a0) {
                return Some(EdgeCross { nature: EdgeCrossType::EndA, point: a0, second: FixVec::ZERO });
            } else {
                return None;
            }
        }

        if a1_area == 0 {
            if other.is_box_contain(a1) {
                return Some(EdgeCross { nature: EdgeCrossType::EndA, point: a1, second: FixVec::ZERO });
            } else {
                return None;
            }
        }

        let b0_area = Triangle::unsafe_area_two(a0, b0, a1);

        if b0_area == 0 {
            if self.is_box_contain(b0) {
                return Some(EdgeCross { nature: EdgeCrossType::EndB, point: b0, second: FixVec::ZERO });
            } else {
                return None;
            }
        }

        let b1_area = Triangle::unsafe_area_two(a0, b1, a1);

        if b1_area == 0 {
            if self.is_box_contain(b1) {
                return Some(EdgeCross { nature: EdgeCrossType::EndB, point: b1, second: FixVec::ZERO });
            } else {
                return None;
            }
        }

        // areas of triangles must have opposite sign
        let area_a = a0_area > 0 && a1_area < 0 || a0_area < 0 && a1_area > 0;
        let area_b = b0_area > 0 && b1_area < 0 || b0_area < 0 && b1_area > 0;

        if !(area_a && area_b) {
            return None;
        }

        let p = FixEdge::cross_point(a0, a1, b0, b1);
        
        // still can be common ends cause rounding
        let end_a = a0 == p || a1 == p;
        let end_b = b0 == p || b1 == p;

        let edge_type;

        if !end_a && !end_b {
            edge_type = EdgeCrossType::Pure;
        } else if end_a {
            edge_type = EdgeCrossType::EndA;
        } else if end_b {
            edge_type = EdgeCrossType::EndB;
        } else {
            panic!("Impossible");
        }

        return Some(EdgeCross { nature: edge_type, point: p, second: FixVec::ZERO });
    }

    fn cross_point(a0: FixVec, a1: FixVec, b0: FixVec, b1: FixVec) -> FixVec {
        // edges are not parralel
        // FixVec(Int64, Int64) where abs(x) and abs(y) < 2^30
        // So the result must be also be in range of 2^30
        
        // Classic aproach:
        
        // let dxA = a0.x - a1.x
        // let dyB = b0.y - b1.y
        // let dyA = a0.y - a1.y
        // let dxB = b0.x - b1.x
        //
        // let xyA = a0.x * a1.y - a0.y * a1.x
        // let xyB = b0.x * b1.y - b0.y * b1.x
        //
        // overflow is possible!
        // let kx = xyA * dxB - dxA * xyB
        //
        // overflow is possible!
        // let ky = xyA * dyB - dyA * xyB
        //
        // let divider = dxA * dyB - dyA * dxB
        //
        // let x = kx / divider
        // let y = ky / divider
        //
        // return FixVec(x, y)

        // offset approach
        // move all picture by -a0. Point a0 will be equal (0, 0)

        // move a0.x to 0
        // move all by a0.x
        let a0x = a0.x.value();
        let a0y = a0.y.value();

        let a1x = a1.x.value() - a0x;
        let b0x = b0.x.value() - a0x;
        let b1x = b1.x.value() - a0x;

        // move a0.y to 0
        // move all by a0.y
        let a1y = a1.y.value() - a0y;
        let b0y = b0.y.value() - a0y;
        let b1y = b1.y.value() - a0y;
        
        let dy_b = b0y - b1y;
        let dx_b = b0x - b1x;
        
     // let xyA = 0
        let xy_b = b0x * b1y - b0y * b1x;
 
        let x0: i64;
        let y0: i64;
        
        // a1y and a1x cannot be zero simultaneously, cause we will get edge a0<>a1 zero length and it is impossible
        
        if a1x == 0 {
            // dxB is not zero cause it will be parallel case and it's impossible
            x0 = 0;
            y0 = xy_b / dx_b;
        } else if a1y == 0 {
            // dyB is not zero cause it will be parallel case and it's impossible
            y0 = 0;
            x0 = -xy_b / dy_b;
        } else {
            // multiply denominator and discriminant by same value to increase precision
            let a1x_128 = a1x as i128;
            let a1y_128 = a1y as i128;
            let kx = a1x_128 * xy_b as i128;
            let ky = a1y_128 * xy_b as i128;

            let divider = a1y_128 * dx_b as i128 - a1x_128 * dy_b as i128;

            x0 = (kx / divider) as i64;
            y0 = (ky / divider) as i64;
        }
        
        let x = x0 + a0x;
        let y = y0 + a0y;
        
        return FixVec::new_i64(x, y)
    }

    pub fn is_box_contain(&self, p: FixVec) -> bool {
        let x = self.e0.x <= p.x && p.x <= self.e1.x || self.e1.x <= p.x && p.x <= self.e0.x;
        let y = self.e0.y <= p.y && p.y <= self.e1.y || self.e1.y <= p.y && p.y <= self.e0.y;
        
        x && y
    }

    fn same_line_overlay(edge_a: FixEdge, edge_b: FixEdge) -> EdgeCross {
        let a = FixBnd::new_two_points(edge_a.e0, edge_a.e1);
        let b = FixBnd::new_two_points(edge_b.e0, edge_b.e1);

        let is_collide = a.is_collide(b);

        if !is_collide {
            return EdgeCross::NOT_CROSS
        }

        let is_a = a.is_inside(b); // b inside a
        let is_b = b.is_inside(a); // a inside b
        
        if is_a && is_b {
            // edges are equal
            return EdgeCross::NOT_CROSS
        }
        
        if is_a {
            // b inside a
            
            let is_be0 = edge_b.e0 == edge_a.e0 || edge_b.e0 == edge_a.e1;
            let is_be1 = edge_b.e1 == edge_a.e0 || edge_b.e1 == edge_a.e1;

            if is_be0 {
                // first point is common
                return EdgeCross{ nature: EdgeCrossType::EndB, point: edge_b.e1, second: FixVec::ZERO }
            } else if is_be1 {
                // second point is common
                return EdgeCross{ nature: EdgeCrossType::EndB, point: edge_b.e0, second: FixVec::ZERO }
            } else {
                // no common points
                return EdgeCross{ nature: EdgeCrossType::OverlayB, point: FixVec::ZERO, second: FixVec::ZERO }
            }
        }
        
        if is_b {
            // a inside b

            let is_ae0 = edge_a.e0 == edge_b.e0 || edge_a.e0 == edge_b.e1;
            let is_ae1 = edge_a.e1 == edge_b.e0 || edge_a.e1 == edge_b.e1;
            
            if is_ae0 {
                // first point is common
                return EdgeCross{ nature: EdgeCrossType::EndA, point: edge_a.e1, second: FixVec::ZERO }
            } else if is_ae1 {
                // second point is common
                return EdgeCross{ nature: EdgeCrossType::EndA, point: edge_a.e0, second: FixVec::ZERO }
            } else {
                // no common points
                return EdgeCross{ nature: EdgeCrossType::OverlayA, point: FixVec::ZERO, second: FixVec::ZERO }
            }
        }
        
        let has_same_end = edge_a.e0 == edge_b.e0 || edge_a.e0 == edge_b.e1 || edge_a.e1 == edge_b.e0 || edge_a.e1 == edge_b.e1;
        
        if has_same_end {
            return EdgeCross::NOT_CROSS
        }
        
        // penetrate
        
        let ap = if a.is_contain(edge_b.e0) { edge_b.e0 } else { edge_b.e1 };
        let bp = if b.is_contain(edge_a.e0) { edge_a.e0 } else { edge_a.e1 };
        
        return EdgeCross { nature: EdgeCrossType::Penetrate, point: ap, second: bp }
    }

}
