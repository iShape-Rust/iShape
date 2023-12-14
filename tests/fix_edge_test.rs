use i_float::fix_vec::FixVec;
use i_shape::fix_edge::{FixEdge, EdgeCrossType};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_cross() {
        let s: i64 = 1024;
        
        let ea = FixEdge::new(FixVec::new_i64(-s, 0), FixVec::new_i64(s, 0));
        let eb = FixEdge::new(FixVec::new_i64(0, -s), FixVec::new_i64(0, s));
        
        let result = ea.cross(eb).unwrap();
        
        assert_eq!(EdgeCrossType::Pure, result.nature);
        assert_eq!(FixVec::ZERO, result.point);
    }

    #[test]
    fn test_big_cross_1() {
        let s: i64 = 1024_000_000;
        
        let ea = FixEdge::new(FixVec::new_i64(-s, 0), FixVec::new_i64(s, 0));
        let eb = FixEdge::new(FixVec::new_i64(0, -s), FixVec::new_i64(0, s));
        
        let result = ea.cross(eb).unwrap();
        
        assert_eq!(EdgeCrossType::Pure, result.nature);
        assert_eq!(FixVec::ZERO, result.point);
    }

    #[test]
    fn test_big_cross_2() {
        let s: i64 = 1024_000_000;
        
        let ea = FixEdge::new(FixVec::new_i64(-s, 0), FixVec::new_i64(s, 0));
        let eb = FixEdge::new(FixVec::new_i64(1024, -s), FixVec::new_i64(1024, s));
        
        let result = ea.cross(eb).unwrap();
        
        assert_eq!(EdgeCrossType::Pure, result.nature);
        assert_eq!(FixVec::new_i64(1024, 0), result.point);
    }

    #[test]
    fn test_big_cross_3() {
        let s: i64 = 1024_000_000;
        let q: i64 = s / 2;
        
        let ea = FixEdge::new(FixVec::new_i64(-s, -s), FixVec::new_i64(s, s));
        let eb = FixEdge::new(FixVec::new_i64(q, -s), FixVec::new_i64(q, s));
        
        let result = ea.cross(eb).unwrap();
        
        assert_eq!(EdgeCrossType::Pure, result.nature);
        assert_eq!(FixVec::new_i64(512_000_000, 512_000_000), result.point);
    }

    #[test]
    fn test_left_end() {
        let s: i64 = 1024_000_000;
        
        let ea = FixEdge::new(FixVec::new_i64(-s, 0), FixVec::new_i64(s, 0));
        let eb = FixEdge::new(FixVec::new_i64(-s, -s), FixVec::new_i64(-s, s));
        
        let result = ea.cross(eb).unwrap();
        
        assert_eq!(EdgeCrossType::EndA, result.nature);
        assert_eq!(FixVec::new_i64(-s, 0), result.point);
    }

    #[test]
    fn test_right_end() {
        let s: i64 = 1024_000_000;
        
        let ea = FixEdge::new(FixVec::new_i64(-s, 0), FixVec::new_i64(s, 0));
        let eb = FixEdge::new(FixVec::new_i64(s, -s), FixVec::new_i64(s, s));
        
        let result = ea.cross(eb).unwrap();
        
        assert_eq!(EdgeCrossType::EndA, result.nature);
        assert_eq!(FixVec::new_i64(s, 0), result.point);
    }

    #[test]
    fn test_left_top() {
        let s: i64 = 1024_000_000;
        
        let ea = FixEdge::new(FixVec::new_i64(-s, s), FixVec::new_i64(s, s));
        let eb = FixEdge::new(FixVec::new_i64(-s, s), FixVec::new_i64(-s, -s));
        
        let result = ea.cross(eb);
        assert!(result.is_none());
    }

    #[test]
    fn test_real_case_1() {
        let ea = FixEdge::new(FixVec::new_i64(7256, -14637), FixVec::new_i64(7454, -15045));
        let eb = FixEdge::new(FixVec::new_i64(7343, -14833), FixVec::new_i64(7506, -15144));
        
        let result = ea.cross(eb).unwrap();
        
        assert!(ea.is_box_contain(result.point));
        assert!(eb.is_box_contain(result.point));
        
        assert_eq!(EdgeCrossType::Pure, result.nature);
    }

    #[test]
    fn test_real_case_2() {
        let ea = FixEdge::new(FixVec::new_i64(-8555798, -1599355), FixVec::new_i64(-1024000, 0));
        let eb = FixEdge::new(FixVec::new_i64(-8571363, 1513719), FixVec::new_i64(-1023948, -10239));
        
        let result = ea.cross(eb).unwrap();
        
        assert_eq!(EdgeCrossType::Pure, result.nature);
        assert_eq!(FixVec::new_i64(-1048691, -5244), result.point);
    }

    #[test]
    fn test_real_case_3() {
        let ea = FixEdge::new(FixVec::new_i64(-8555798, -1599355), FixVec::new_i64(513224, -5243));
        let eb = FixEdge::new(FixVec::new_i64(-8555798, -1599355), FixVec::new_i64(513224, -5243));
        
        let result = ea.cross(eb);

        assert!(result.is_none());
    }

    #[test]
    fn test_penetration() {
        let s: i64 = 1024;
        
        let ea = FixEdge::new(FixVec::new_i64(-s, 0), FixVec::new_i64(s / 2, 0));
        let eb = FixEdge::new(FixVec::new_i64(0, 0), FixVec::new_i64(s, 0));
        
        let result = ea.cross(eb).unwrap();
        
        assert_eq!(EdgeCrossType::Penetrate, result.nature);
        assert_eq!(FixVec::ZERO, result.point);
        assert_eq!(FixVec::new_i64(512, 0), result.second);
    }

    #[test]
    fn test_full_overlay() {
        let ea = FixEdge::new(FixVec::new_i64(-2, 0), FixVec::new_i64(2, 0));
        let eb = FixEdge::new(FixVec::new_i64(-1, 0), FixVec::new_i64(1, 0));
        
        let result = ea.cross(eb).unwrap();
        
        assert_eq!(EdgeCrossType::OverlayB, result.nature);
    }
    

}