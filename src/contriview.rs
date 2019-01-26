#[derive(Debug, Default, PartialEq, Eq)]
pub struct ContriView {
    today_contributions: u32,
    week_contributions: u32,
    month_contributions: u32,
    sum_contributions: u32,
}

impl ContriView {
    fn sum_contributions_from_html(html: &str) -> u32 {
        let doc = Html::parse_document(&html);
        let selector = Selector::parse(r#"rect[data-date]"#).unwrap();
        let input = doc.select(&selector);

        input
            .into_iter()
            .map(|i| -> u32 {
                let contribution = i.value().attr("data-count").unwrap();
                contribution.parse().unwrap_or_default()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::*;

    #[test]
    fn contriview_default() {
        assert_eq!(
            ContriView::default(),
            ContriView {
                today_contributions: 0,
                week_contributions: 0,
                month_contributions: 0,
                sum_contributions: 0
            }
        )
    }

    #[test]
    fn test_sum_contributions() {
        assert_eq!(
            3532,
            ContriView::sum_contributions_from_html(&sample_html()),
        )
    }
}
