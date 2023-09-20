use geo_types::geometry::*;

use crate::{
    errors::Result,
    transform::{Transform, TransformClosure},
};

impl Transform for Coord {
    fn transform_coordinates<F: TransformClosure>(&mut self, f: &mut F) -> Result<()> {
        let mut xy = (self.x, self.y);
        (&mut xy).transform_coordinates(f)?;
        *self = Coord::from(xy);
        Ok(())
    }
}

impl Transform for Point {
    fn transform_coordinates<F: TransformClosure>(&mut self, f: &mut F) -> Result<()> {
        self.0.transform_coordinates(f)
    }
}

impl Transform for MultiPoint {
    fn transform_coordinates<F: TransformClosure>(&mut self, f: &mut F) -> Result<()> {
        self.iter_mut()
            .try_for_each(|point| point.transform_coordinates(f))
    }
}

impl Transform for Line {
    fn transform_coordinates<F: TransformClosure>(&mut self, f: &mut F) -> Result<()> {
        let (mut start, mut end) = self.points();
        start.transform_coordinates(f)?;
        end.transform_coordinates(f)?;
        *self = Line::new(start, end);
        Ok(())
    }
}

impl Transform for LineString {
    fn transform_coordinates<F: TransformClosure>(&mut self, f: &mut F) -> Result<()> {
        self.coords_mut()
            .try_for_each(|coord| coord.transform_coordinates(f))
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::{transform::transform, Proj};

    use super::*;

    const X_0: f64 = 0.03490658503988659;
    const Y_0: f64 = 0.017453292519943295;
    const COORD_0: Coord = Coord { x: X_0, y: Y_0 };

    const X_1: f64 = 222650.79679758527;
    const Y_1: f64 = 110642.22941193319;
    const COORD_1: Coord = Coord { x: X_1, y: Y_1 };

    const EPS: f64 = 1.0e-10;

    #[test]
    fn transforms_coord() {
        let mut coord = COORD_0;
        transform_helper(&mut coord);
        assert_cord_eq(COORD_1, coord)
    }

    #[test]
    fn transforms_point() {
        let mut point = Point::from(COORD_0);
        transform_helper(&mut point);
        assert_cord_eq(COORD_1, point.0)
    }

    #[test]
    fn transforms_multi_point() {
        let mut multi_point: MultiPoint = (0..10).map(|_| Point::from(COORD_0)).collect();
        transform_helper(&mut multi_point);
        multi_point
            .iter()
            .for_each(|point| assert_cord_eq(COORD_1, point.0));
    }

    #[test]
    fn transforms_line() {
        let mut line = Line::new(-COORD_0, COORD_0);
        transform_helper(&mut line);
        assert_cord_eq(-COORD_1, line.start);
        assert_cord_eq(COORD_1, line.end);
    }

    #[test]
    fn transforms_line_string() {
        let mut line_string = LineString::new(vec![-COORD_0, COORD_0]);
        transform_helper(&mut line_string);
        assert_cord_eq(-COORD_1, line_string.0[0]);
        assert_cord_eq(COORD_1, line_string.0[1]);
    }

    fn transform_helper<T: Transform>(geometry: &mut T) {
        let from = Proj::from_proj_string("+proj=latlong +ellps=GRS80").unwrap();
        let to = Proj::from_proj_string("+proj=etmerc +ellps=GRS80").unwrap();
        transform(&from, &to, geometry).unwrap();
    }

    fn assert_cord_eq(expected_coord: Coord, actual_coord: Coord) {
        assert_abs_diff_eq!(expected_coord.x, actual_coord.x, epsilon = EPS);
        assert_abs_diff_eq!(expected_coord.y, actual_coord.y, epsilon = EPS);
    }
}
