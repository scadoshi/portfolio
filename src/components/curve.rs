//! A smooth path through measured points, shared by the charts.

use std::fmt::Write as _;

/// A smooth path through `points`: Catmull-Rom turned into cubic curves, which
/// passes through every point rather than near it. Empty for no points.
pub fn curve(points: &[(f64, f64)]) -> String {
    let Some(first) = points.first() else {
        return String::new();
    };
    let mut d = format!("M {:.1} {:.1}", first.0, first.1);
    for i in 0..points.len().saturating_sub(1) {
        let p0 = points[i.saturating_sub(1)];
        let p1 = points[i];
        let p2 = points[i + 1];
        let p3 = points[(i + 2).min(points.len() - 1)];
        let c1 = (p1.0 + (p2.0 - p0.0) / 6.0, p1.1 + (p2.1 - p0.1) / 6.0);
        let c2 = (p2.0 - (p3.0 - p1.0) / 6.0, p2.1 - (p3.1 - p1.1) / 6.0);
        let _ = write!(
            d,
            " C {:.1} {:.1}, {:.1} {:.1}, {:.1} {:.1}",
            c1.0, c1.1, c2.0, c2.1, p2.0, p2.1
        );
    }
    d
}

/// [`curve`] closed down to `baseline`, for the area under a line.
pub fn area(points: &[(f64, f64)], baseline: f64) -> String {
    match (points.first(), points.last()) {
        (Some(first), Some(last)) => format!(
            "{} L {:.1} {baseline:.1} L {:.1} {baseline:.1} Z",
            curve(points),
            last.0,
            first.0
        ),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_curve_starts_at_the_first_point_and_the_area_closes_on_the_baseline() {
        let points = [(0.0, 10.0), (10.0, 0.0), (20.0, 10.0)];
        assert!(curve(&points).starts_with("M 0.0 10.0 C"));
        assert_eq!(curve(&points).matches(" C ").count(), 2);
        let area = area(&points, 12.0);
        assert!(area.ends_with("L 20.0 12.0 L 0.0 12.0 Z"), "{area}");
        assert_eq!(curve(&[]), "");
    }
}
