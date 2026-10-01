//! A smooth path through measured points, shared by the charts.

use std::fmt::Write as _;

/// A smooth path through `points` that passes through every point and never
/// overshoots between two of them: Fritsch-Carlson monotone cubic
/// interpolation as cubic Bezier segments. A flat run into a spike stays flat
/// rather than dipping below the data first. Empty for no points.
pub fn curve(points: &[(f64, f64)]) -> String {
    let Some(first) = points.first() else {
        return String::new();
    };
    let mut d = format!("M {:.1} {:.1}", first.0, first.1);
    let n = points.len();
    if n < 2 {
        return d;
    }
    // Secant slopes between neighbors, then a tangent per point that keeps
    // each segment monotone.
    let deltas: Vec<f64> = points
        .windows(2)
        .map(|pair| {
            let dx = pair[1].0 - pair[0].0;
            if dx.abs() < f64::EPSILON {
                0.0
            } else {
                (pair[1].1 - pair[0].1) / dx
            }
        })
        .collect();
    let mut tangents = vec![0.0; n];
    tangents[0] = deltas[0];
    tangents[n - 1] = deltas[n - 2];
    for i in 1..n - 1 {
        let (a, b) = (deltas[i - 1], deltas[i]);
        tangents[i] = if a * b <= 0.0 {
            0.0
        } else {
            f64::midpoint(a, b)
        };
    }
    for i in 0..n - 1 {
        if deltas[i].abs() < f64::EPSILON {
            tangents[i] = 0.0;
            tangents[i + 1] = 0.0;
            continue;
        }
        let alpha = tangents[i] / deltas[i];
        let beta = tangents[i + 1] / deltas[i];
        let size = alpha.hypot(beta);
        if size > 3.0 {
            let scale = 3.0 / size;
            tangents[i] = scale * alpha * deltas[i];
            tangents[i + 1] = scale * beta * deltas[i];
        }
    }
    for i in 0..n - 1 {
        let (p1, p2) = (points[i], points[i + 1]);
        let dx = (p2.0 - p1.0) / 3.0;
        let c1 = (p1.0 + dx, p1.1 + tangents[i] * dx);
        let c2 = (p2.0 - dx, p2.1 - tangents[i + 1] * dx);
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

    /// Every y in a path's control points and anchors.
    fn ys(path: &str) -> Vec<f64> {
        path.split([',', ' '])
            .filter_map(|token| token.parse::<f64>().ok())
            .skip(1)
            .step_by(2)
            .collect()
    }

    #[test]
    fn the_curve_starts_at_the_first_point_and_the_area_closes_on_the_baseline() {
        let points = [(0.0, 10.0), (10.0, 0.0), (20.0, 10.0)];
        assert!(curve(&points).starts_with("M 0.0 10.0 C"));
        assert_eq!(curve(&points).matches(" C ").count(), 2);
        let area = area(&points, 12.0);
        assert!(area.ends_with("L 20.0 12.0 L 0.0 12.0 Z"), "{area}");
        assert_eq!(curve(&[]), "");
    }

    #[test]
    fn a_flat_run_into_a_spike_never_dips_below_the_flat() {
        // In SVG a larger y is lower on the page: the baseline is 100 and the
        // spike rises to 10. No control point may go past 100.
        let points = [
            (0.0, 100.0),
            (10.0, 100.0),
            (20.0, 100.0),
            (30.0, 10.0),
            (40.0, 100.0),
        ];
        let lowest = ys(&curve(&points)).into_iter().fold(0.0_f64, f64::max);
        assert!(lowest <= 100.0, "a control point overshot to {lowest}");
    }
}
