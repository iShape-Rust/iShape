use crate::base::data::Path;
use alloc::vec::Vec;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::rect::{FloatRect, FloatRectError};

/// Builds bounds, returning `Ok(None)` for empty input and an error for invalid coordinates.
pub trait RectInit<P>
where
    P: FloatPointCompatible,
{
    fn with_path(path: &[P]) -> Result<Option<FloatRect<P::Scalar>>, FloatRectError>;
    fn with_paths(paths: &[Vec<P>]) -> Result<Option<FloatRect<P::Scalar>>, FloatRectError>;
    fn with_list_of_paths(list: &[Vec<Vec<P>>]) -> Result<Option<FloatRect<P::Scalar>>, FloatRectError>;
}

impl<P> RectInit<P> for FloatRect<P::Scalar>
where
    P: FloatPointCompatible,
{
    fn with_path(path: &[P]) -> Result<Option<FloatRect<P::Scalar>>, FloatRectError> {
        Self::with_iter(path.iter())
    }

    fn with_paths(paths: &[Path<P>]) -> Result<Option<FloatRect<P::Scalar>>, FloatRectError> {
        Self::with_iter(paths.iter().flatten())
    }

    fn with_list_of_paths(list: &[Vec<Path<P>>]) -> Result<Option<FloatRect<P::Scalar>>, FloatRectError> {
        Self::with_iter(list.iter().flatten().flatten())
    }
}
