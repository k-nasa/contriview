use chrono::*;
use failure::Error;
use scraper::*;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ContriView {
    today_contributions: u32,
    week_contributions: u32,
    month_contributions: u32,
    sum_contributions: u32,
}

impl ContriView {
    pub fn from_html(html: &str) -> Result<Self, Error> {
        let sum_contributions = Self::sum_contributions_from_html(html);
        let today_contributions = Self::today_contributions_from_html(html);
        let week_contributions = Self::week_contributions_from_html(html);
        let month_contributions = Self::month_contributions_from_html(html);

        Ok(ContriView {
            sum_contributions,
            week_contributions,
            month_contributions,
            today_contributions,
        })
    }

    // FIXME use Result
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

    // FIXME use Result
    fn month_contributions_from_html(html: &str) -> u32 {
        let doc = Html::parse_document(&html);

        let now = Local::now().format("%Y-%m").to_string();
        let selector = format!("rect[data-date^=\"{}\"]", now);

        let selector = Selector::parse(&selector).unwrap();
        let input = doc.select(&selector);

        let contributions: Vec<u32> = input
            .into_iter()
            .map(|i| -> u32 {
                let contribution = i.value().attr("data-count").unwrap();
                contribution.parse().unwrap_or_default()
            })
            .collect();

        contributions.iter().sum()
    }

    // FIXME use Result
    fn week_contributions_from_html(html: &str) -> u32 {
        let doc = Html::parse_document(&html);
        let selector = Selector::parse(r#"rect[data-date]"#).unwrap();
        let input = doc.select(&selector);

        let contributions: Vec<u32> = input
            .into_iter()
            .map(|i| -> u32 {
                let contribution = i.value().attr("data-count").unwrap();
                contribution.parse().unwrap_or_default()
            })
            .collect();

        contributions.iter().rev().take(7).sum()
    }

    // FIXME use Result
    fn today_contributions_from_html(html: &str) -> u32 {
        let doc = Html::parse_document(&html);

        let now = Local::now().format("%Y-%m-%d").to_string();
        let selector = format!("rect[data-date=\"{}\"]", now);

        let selector = Selector::parse(&selector).unwrap();
        let input = doc.select(&selector).next().unwrap();

        input.value().attr("data-count").unwrap().parse().unwrap()
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
    fn test_from_html() {
        assert_eq!(
            ContriView::from_html(&sample_html()).unwrap(),
            ContriView {
                sum_contributions: 3532,
                month_contributions: 260,
                week_contributions: 51,
                today_contributions: 3
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

    #[test]
    fn test_week_contributions() {
        assert_eq!(51, ContriView::week_contributions_from_html(&sample_html()),)
    }

    #[test]
    fn test_today_contributions() {
        assert_eq!(3, ContriView::today_contributions_from_html(&sample_html()),)
    }

    #[test]
    fn test_month_contributions() {
        assert_eq!(
            260,
            ContriView::month_contributions_from_html(&sample_html()),
        )
    }
}
