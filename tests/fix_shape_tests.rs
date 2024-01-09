use i_float::fix_vec::FixVec;
use i_shape::fix_shape::FixShape;


#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn test_one_hole() {
        let body = [
            FixVec::new_number(0, 0),
            FixVec::new_number(0, 2),
            FixVec::new_number(2, 2),
            FixVec::new_number(2, 0)
        ];

        let hole = [
            FixVec::new_number(0, 0),
            FixVec::new_number(1, 0),
            FixVec::new_number(1, 1),
            FixVec::new_number(0, 1)
        ];

        let mut holes = Vec::with_capacity(1);
        holes.push(hole.to_vec());

        let shape = FixShape::new_with_contour_and_holes(body.to_vec(), holes);
        
        assert_eq!(shape.contour().as_slice(), body.as_ref());
        assert_eq!(shape.holes()[0].as_slice(), hole.as_ref());
    }

    #[test]
    fn test_multiple_holes() {
        let body = vec![
            FixVec::new_number(0, 0),
            FixVec::new_number(0, 4),
            FixVec::new_number(4, 4),
            FixVec::new_number(4, 0),
        ];

        let hole1 = vec![
            FixVec::new_number(0, 0),
            FixVec::new_number(1, 0),
            FixVec::new_number(1, 1),
            FixVec::new_number(0, 1),
        ];

        let hole2 = vec![
            FixVec::new_number(2, 2),
            FixVec::new_number(3, 2),
            FixVec::new_number(3, 3),
            FixVec::new_number(2, 3),
        ];

        let holes = vec![hole1.clone(), hole2.clone()];

        let shape = FixShape::new_with_contour_and_holes(body.clone(), holes);

        assert_eq!(shape.contour().as_slice(), body.as_slice());
        assert_eq!(shape.holes()[0].as_slice(), hole1.as_slice());
        assert_eq!(shape.holes()[1].as_slice(), hole2.as_slice());
    }

    #[test]
    fn test_add_hole() {
        let body = vec![
            FixVec::new_number(0, 0),
            FixVec::new_number(0, 2),
            FixVec::new_number(2, 2),
            FixVec::new_number(2, 0),
        ];

        let hole = vec![
            FixVec::new_number(0, 0),
            FixVec::new_number(1, 0),
            FixVec::new_number(1, 1),
            FixVec::new_number(0, 1),
        ];

        let mut shape = FixShape::new_with_contour_and_holes(body, Vec::new());

        shape.add_hole(hole.clone());

        assert_eq!(shape.holes()[0].as_slice(), hole.as_slice());
    }

}