use i_float::fix_vec::FixVec;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IndexPoint {
    pub index: i64,
    pub point: FixVec
}

impl IndexPoint {

    pub fn new(index: int, point: FixVec) {
        IndexPoint {index, point}
    }

}