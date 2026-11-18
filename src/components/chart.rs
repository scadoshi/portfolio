use dioxus::prelude::*;
use orangui::{HeatScrollParts, use_open_on_newest};

/// What a chart's hover chip says and where it sits, as percentages of the
/// chart's width and height so it follows the chart at any size.
#[derive(Clone, PartialEq)]
pub(crate) struct Tip {
    pub text: String,
    pub left: f64,
    pub top: f64,
}

pub(crate) fn show(mut tip: Signal<Option<Tip>>, text: String, left: f64, top: f64) {
    tip.set(Some(Tip { text, left, top }));
}

/// The pinned-axis charts' parts the open-on-newest script reads.
const SCROLL_PARTS: HeatScrollParts = HeatScrollParts {
    scroll: ".axis-scroll",
    row: ".chart-row",
    axis: ".chart-axis",
    month: ".heatmap-month",
};

/// Opens every `.axis-scroll` chart wider than its box on its newest columns, a beat after
/// mount so the boxes have been laid out. A month label the pinned axis cuts through is moved
/// on under the axis, with that much room added at the far end, so the newest week stays in
/// view and no label shows by half at the edge.
pub(crate) fn use_scroll_to_latest() {
    use_open_on_newest(SCROLL_PARTS);
}

/// `Jan` for `01`, the month of a `YYYY-MM-DD` date.
pub(crate) fn month_name(month: &str) -> Option<&'static str> {
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
    fn months_come_from_their_two_digits() {
        assert_eq!(month_name("01"), Some("Jan"));
        assert_eq!(month_name("12"), Some("Dec"));
        assert_eq!(month_name("13"), None);
    }
}
