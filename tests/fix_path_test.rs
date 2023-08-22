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
            FixVec::new_i64(512, 1024),
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
            FixVec::new_i64(0, 512),
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

        let area_1 = path.to_vec().area();
        path.reverse();

        let area_2 = path.to_vec().area();

        assert_eq!(area_1, 1024);
        assert_eq!(area_2, -1024);
    }

}