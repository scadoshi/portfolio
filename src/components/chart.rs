use dioxus::prelude::*;

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

/// Opens every `.axis-scroll` chart wider than its box on its newest columns.
/// A month label the pinned axis cuts through is moved on under the axis,
/// with that much room added at the far end, so the newest week stays in
/// view and no label shows by half at the edge.
const SCROLL_TO_LATEST_JS: &str = r"
setTimeout(() => {
    for (const box of document.querySelectorAll('.axis-scroll')) {
        const row = box.querySelector('.chart-row');
        const axis = box.querySelector('.chart-axis');
        if (!row || !axis) continue;
        row.style.paddingRight = '';
        box.scrollLeft = box.scrollWidth;
        if (box.scrollWidth <= box.clientWidth) continue;
        const edge = axis.getBoundingClientRect().right;
        const cut = Array.from(box.querySelectorAll('.heatmap-month'))
            .map((label) => label.getBoundingClientRect())
            .find((r) => r.left < edge && r.right > edge);
        if (!cut) continue;
        row.style.paddingRight = Math.ceil(cut.right - edge) + 2 + 'px';
        box.scrollLeft = box.scrollWidth;
    }
}, 60);
";

/// Opens the page's pinned-axis charts on their newest columns, a beat after
/// mount so the boxes have been laid out.
pub(crate) fn use_scroll_to_latest() {
    use_effect(|| {
        document::eval(SCROLL_TO_LATEST_JS);
    });
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
