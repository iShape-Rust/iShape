use i_float::fix_vec::FixVec;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IndexPoint {
    pub index: usize,
    pub point: FixVec
}

impl IndexPoint {

    pub const ZERO: Self = Self { index: 0, point: FixVec::ZERO };

    pub fn new(index: usize, point: FixVec) -> IndexPoint {
        IndexPoint { index, point }
    }

}