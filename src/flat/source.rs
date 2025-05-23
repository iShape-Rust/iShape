use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use crate::flat::buffer::FlatContoursBuffer;
use crate::source::resource::ShapeResource;

impl FlatContoursBuffer {

    #[inline]
    pub fn set_with_resource<P, T, R>(&mut self, resource: &R) -> FloatPointAdapter<P, T>
    where
        T: FloatNumber,
        P: FloatPointCompatible<T>,
        R: ShapeResource<P, T> +?Sized
    {
        let contours_count = resource.iter_paths().count();
        let points_count = resource.iter_paths().fold(0, |s, contour| s + contour.len());

        let adapter = FloatPointAdapter::with_iter(resource.iter_paths().flatten());

        self.clear_and_reserve(points_count, contours_count);
        let mut offset = 0;
        for contour in resource.iter_paths() {
            self.points.extend(contour.iter().map(|p|adapter.float_to_int(p)));
            let contour_len = contour.len();
            self.ranges.push(offset..contour_len);
            offset += contour_len;
        }

        adapter
    }
}