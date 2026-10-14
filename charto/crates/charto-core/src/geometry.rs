//! Geometry helpers ported from `sconcho/util/canvas.py`.

pub type PointF = (f64, f64);
pub type Segment = (PointF, PointF);

/// Lexicographic "less than" on points, matching Python tuple ordering.
fn point_lt(a: PointF, b: PointF) -> bool {
    a.0 < b.0 || (a.0 == b.0 && a.1 < b.1)
}

/// Smallest point over all segment end points.
fn min_point(segments: &[Segment]) -> Option<PointF> {
    let first = segments.first()?;
    let mut current = first.0;
    for seg in segments {
        if point_lt(seg.0, current) {
            current = seg.0;
        }
        if point_lt(seg.1, current) {
            current = seg.1;
        }
    }
    Some(current)
}

/// Sorts the line segments of a pattern repeat outline into a consecutive
/// list of vertices (the last vertex repeats the first, as in the original).
///
/// Returns `None` if the segments do not form one connected outline
/// (e.g. the repeat area has a hole) or if there are no segments.
pub fn sort_vertices(mut segments: Vec<Segment>) -> Option<Vec<PointF>> {
    let start = min_point(&segments)?;
    let mut vertices = vec![start];
    let mut current = start;

    while !segments.is_empty() {
        let idx = segments
            .iter()
            .position(|s| s.0 == current || s.1 == current)?;
        let seg = segments.remove(idx);
        if seg.0 == current {
            vertices.push(seg.1);
            current = seg.1;
        } else {
            vertices.push(seg.0);
            current = seg.0;
        }
    }
    Some(vertices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_outline() {
        let segs = vec![
            ((0.0, 0.0), (1.0, 0.0)),
            ((1.0, 0.0), (1.0, 1.0)),
            ((1.0, 1.0), (0.0, 1.0)),
            ((0.0, 1.0), (0.0, 0.0)),
        ];
        let v = sort_vertices(segs).unwrap();
        assert_eq!(
            v,
            vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (0.0, 0.0)]
        );
    }

    #[test]
    fn disconnected_is_none() {
        let segs = vec![((0.0, 0.0), (1.0, 0.0)), ((5.0, 5.0), (6.0, 5.0))];
        assert!(sort_vertices(segs).is_none());
    }

    #[test]
    fn empty_is_none() {
        assert!(sort_vertices(vec![]).is_none());
    }
}
