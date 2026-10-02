use dioxus::prelude::*;

/// What a chart's hover chip says and where it sits, as percentages of the
/// chart's width and height so it follows the chart at any size.
#[derive(Clone, PartialEq)]
pub struct Tip {
    pub text: String,
    pub left: f64,
    pub top: f64,
}

/// How many times the home chart has been asked to animate again. The chart
/// keys its drawing on it, so a bump remounts the SVG and every CSS animation
/// runs from the start.
#[derive(Clone, Copy)]
pub struct Replay(pub Signal<u32>);

pub fn show(mut tip: Signal<Option<Tip>>, text: String, left: f64, top: f64) {
    tip.set(Some(Tip { text, left, top }));
}

/// Which way the chip hangs off its point: centered in the middle of the
/// chart, and from its edge near either side so it never leaves the panel.
pub fn anchor(left: f64) -> &'static str {
    if left < 15.0 {
        "tip-start"
    } else if left > 85.0 {
        "tip-end"
    } else {
        ""
    }
}

/// `Jan` for `01`, the month of a `YYYY-MM-DD` date.
pub fn month_name(month: &str) -> Option<&'static str> {
    Some(match month {
        "01" => "Jan",
        "02" => "Feb",
        "03" => "Mar",
        "04" => "Apr",
        "05" => "May",
        "06" => "Jun",
        "07" => "Jul",
        "08" => "Aug",
        "09" => "Sep",
        "10" => "Oct",
        "11" => "Nov",
        "12" => "Dec",
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_chip_hangs_from_its_edge_near_either_side() {
        assert_eq!(anchor(5.0), "tip-start");
        assert_eq!(anchor(50.0), "");
        assert_eq!(anchor(95.0), "tip-end");
    }

    #[test]
    fn months_come_from_their_two_digits() {
        assert_eq!(month_name("01"), Some("Jan"));
        assert_eq!(month_name("12"), Some("Dec"));
        assert_eq!(month_name("13"), None);
    }
}
