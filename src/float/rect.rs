use i_float::float::Float;
use i_float::float_point::{FloatPoint, FloatPointCompatible};
use i_float::float_rect::FloatRect;

pub trait RectInit<P, T>
where
    P: FloatPointCompatible<T>,
    T: Float,
{
    fn with_path(path: &[P]) -> Option<FloatRect<T>>;
    fn with_paths(paths: &[Vec<P>]) -> Option<FloatRect<T>>;
    fn with_list_of_paths(list: &[Vec<Vec<P>>]) -> Option<FloatRect<T>>;
}

trait FirstPoint<P, T>
where
    P: FloatPointCompatible<T>,
    T: Float,
{
    fn first_point(&self) -> Option<FloatPoint<T>>;
}

impl<P, T> RectInit<P, T> for FloatRect<T>
where
    P: FloatPointCompatible<T>,
    T: Float,
{
    fn with_path(path: &[P]) -> Option<FloatRect<T>> {
        let first_point = path.first()?.to_float_point();

        let mut rect = Self::with_point(first_point);

        for p in path.iter() {
            rect.unsafe_add_point(p.to_float_point());
        }

        Some(rect)
    }

    fn with_paths(paths: &[Vec<P>]) -> Option<FloatRect<T>> {
        let first_point = paths.first_point()?;

        let mut rect = Self::with_point(first_point);

        for path in paths.iter() {
            for p in path.iter() {
                rect.unsafe_add_point(p.to_float_point());
            }
        }

        Some(rect)
    }

    fn with_list_of_paths(list: &[Vec<Vec<P>>]) -> Option<FloatRect<T>> {
        let first_point = list.first_point()?;

        let mut rect = Self::with_point(first_point);

        for paths in list.iter() {
            for path in paths.iter() {
                for p in path.iter() {
                    rect.unsafe_add_point(p.to_float_point());
                }
            }
        }

        Some(rect)
    }
}

impl<P, T> FirstPoint<P, T> for [Vec<P>]
where
    P: FloatPointCompatible<T>,
    T: Float,
{
    fn first_point(&self) -> Option<FloatPoint<T>> {
        for path in self.iter() {
            if let Some(p) = path.first() {
                return Some(p.to_float_point());
            }
        }
        None
    }
}

impl<P, T> FirstPoint<P, T> for [Vec<Vec<P>>]
where
    P: FloatPointCompatible<T>,
    T: Float,
{
    fn first_point(&self) -> Option<FloatPoint<T>> {
        for paths in self.iter() {
            for path in paths.iter() {
                if let Some(p) = path.first() {
                    return Some(p.to_float_point());
                }
            }
        }
        None
    }
}
