use dioxus::prelude::*;

/// What a chart's hover chip says and where it sits, as percentages of the
/// chart's width and height so it follows the chart at any size.
#[derive(Clone, PartialEq)]
pub struct Tip {
    pub text: String,
    pub left: f64,
    pub top: f64,
}

pub fn show(mut tip: Signal<Option<Tip>>, text: String, left: f64, top: f64) {
    tip.set(Some(Tip { text, left, top }));
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
    fn months_come_from_their_two_digits() {
        assert_eq!(month_name("01"), Some("Jan"));
        assert_eq!(month_name("12"), Some("Dec"));
        assert_eq!(month_name("13"), None);
    }
}
