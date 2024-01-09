use i_float::fix_vec::FixVec;


#[cfg(test)]
mod tests {
    use i_shape::fix_path::FixPathExtension;

    use super::*;
    
    #[test]
    fn test_no_degenerates() {
        let origin = [
            FixVec::new_number(0, 0),
            FixVec::new_number(0, 1),
            FixVec::new_number(1, 1),
            FixVec::new_number(1, 0)
        ];

        let mut path = origin.to_vec();

        path.remove_degenerates();
        
        assert_eq!(path.as_slice(), origin.as_ref(), "The path and origin are not equal!");
    }

    #[test]
    fn test_degenerates() {
        let origin = [
            FixVec::new_number(0, 0),
            FixVec::new_number(0, 1),
            FixVec::new_number(1, 1),
            FixVec::new_number(1, 0)
        ];

        let incorrect = [
            FixVec::new_number(0, 0),
            FixVec::new_number(0, 1),
            FixVec::new_number(0, 1),
            FixVec::new(512, 1024),
            FixVec::new_number(1, 1),
            FixVec::new_number(1, 1),
            FixVec::new_number(1, 0)
        ];

        let mut path = incorrect.to_vec();

        path.remove_degenerates();
        
        assert_eq!(path.as_slice(), origin.as_ref(), "The path and origin are not equal!");
    }

    #[test]
    fn test_degenerates_empty() {
        let incorrect = [
            FixVec::new_number(0, 0),
            FixVec::new(0, 512),
            FixVec::new_number(0, 1)
        ];

        let mut path = incorrect.to_vec();

        path.remove_degenerates();
        
        assert_eq!(path.len(), 0);
    }

    #[test]
    fn test_area_1() {
        let mut path = [
            FixVec::new_number(0, 0),
            FixVec::new_number(0, 1),
            FixVec::new_number(1, 1),
            FixVec::new_number(1, 0)
        ];

        let area_1 = path.to_vec().fix_area();
        path.reverse();

        let area_2 = path.to_vec().fix_area();

        assert_eq!(area_1, 1024);
        assert_eq!(area_2, -1024);
    }

    #[test]
    fn test_order_00() {
        let mut path = [
            FixVec::new_number(-10, -10),
            FixVec::new_number(-10, 10),
            FixVec::new_number(10, 10),
            FixVec::new_number(10, -10)
        ].to_vec();

        assert_eq!(path.is_clockwise_ordered(), true);
    }

    #[test]
    fn test_order_01() {
        let mut path = [
            FixVec::new_number(-10, -10),
            FixVec::new_number(10, -10),
            FixVec::new_number(10, 10),
            FixVec::new_number(-10, 10)
        ].to_vec();

        assert_eq!(path.is_clockwise_ordered(), false);
    }

    #[test]
    fn test_convex_00() {
        let mut path = [
            FixVec::new_number(-10, -10),
            FixVec::new_number(-10, 10),
            FixVec::new_number(10, 10),
            FixVec::new_number(10, -10)
        ].to_vec();

        assert_eq!(path.is_convex(), true);
        path.reverse();
        assert_eq!(path.is_convex(), true);
    }

    #[test]
    fn test_convex_01() {
        let mut path = [
            FixVec::new_number(-10, -10),
            FixVec::new_number(0, 10),
            FixVec::new_number(10, -10),
            FixVec::new_number(0, -5)
        ].to_vec();

        assert_eq!(path.is_convex(), false);
        path.reverse();
        assert_eq!(path.is_convex(), false);
    }

    #[test]
    fn test_convex_02() {
        let mut path = [
            FixVec::new_number(0, 0),
            FixVec::new_number(1, 2),
            FixVec::new_number(3, 3),
            FixVec::new_number(4, 1),
            FixVec::new_number(2, 0)
        ].to_vec();

        assert_eq!(path.is_convex(), true);
        path.reverse();
        assert_eq!(path.is_convex(), true);
    }

    #[test]
    fn test_convex_03() {
        let mut path = [
            FixVec::new_number(0, 0),
            FixVec::new_number(1, 2),
            FixVec::new_number(0, 4),
            FixVec::new_number(4, 2),
            FixVec::new_number(2, 0)
        ].to_vec();

        assert_eq!(path.is_convex(), false);
        path.reverse();
        assert_eq!(path.is_convex(), false);
    }

    #[test]
    fn test_convex_04() {
        let path1 = [
            FixVec::new_number(0, 0)
        ].to_vec();

        assert_eq!(path1.is_convex(), true);

        let path2 = [
            FixVec::new_number(0, 0),
            FixVec::new_number(1, 0)
        ].to_vec();

        assert_eq!(path2.is_convex(), true);
    }

    #[test]
    fn test_convex_05() {
        let mut path = [
            FixVec::new_number(0, 0),
            FixVec::new_number(1, 2),
            FixVec::new_number(2, 3),
            FixVec::new_number(3, 2),
            FixVec::new_number(4, 1),
            FixVec::new_number(2, 0)
        ].to_vec();

        assert_eq!(path.is_convex(), true);
        path.reverse();
        assert_eq!(path.is_convex(), true);
    }

    #[test]
    fn test_convex_06() {
        let mut path = [
            FixVec::new_number(-10, -10),
            FixVec::new_number(-10, 0),
            FixVec::new_number(-10, 10),
            FixVec::new_number(0, 10),
            FixVec::new_number(10, 10),
            FixVec::new_number(10, 0),
            FixVec::new_number(10, -10),
            FixVec::new_number(0, -10)
        ].to_vec();

        assert_eq!(path.is_convex(), true);
        path.reverse();
        assert_eq!(path.is_convex(), true);
    }

    #[test]
    fn test_convex_07() {
        let mut path = [
            FixVec::new_number(-10, -10),
            FixVec::new_number(-9, 0),
            FixVec::new_number(-10, 10),
            FixVec::new_number(0, 10),
            FixVec::new_number(10, 10),
            FixVec::new_number(10, 0),
            FixVec::new_number(10, -10),
            FixVec::new_number(0, -10)
        ].to_vec();

        assert_eq!(path.is_convex(), false);
        path.reverse();
        assert_eq!(path.is_convex(), false);
    }
}