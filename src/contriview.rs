use chrono::*;
use failure::Error;
use scraper::*;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ContriView {
    today_contributions: u32,
    week_contributions: u32,
    month_contributions: u32,
    year_contributions: u32,
    sum_contributions: u32,
}

impl ContriView {
    pub fn from_html(html: &str, date: Date<Local>) -> Result<Self, Error> {
        let sum_contributions = Self::sum_contributions_from_html(html);
        let week_contributions = Self::week_contributions_from_html(html);
        let year_contributions = Self::year_contributions_from_html(html, date);
        let month_contributions = Self::month_contributions_from_html(html, date);
        let today_contributions = Self::today_contributions_from_html(html, date);

        Ok(ContriView {
            sum_contributions,
            week_contributions,
            month_contributions,
            year_contributions,
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
    fn month_contributions_from_html(html: &str, date: Date<Local>) -> u32 {
        let doc = Html::parse_document(&html);

        let now = date.format("%Y-%m").to_string();
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

    fn year_contributions_from_html(html: &str, date: Date<Local>) -> u32 {
        let doc = Html::parse_document(&html);

        let now = date.format("%Y-").to_string();
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
    fn today_contributions_from_html(html: &str, date: Date<Local>) -> u32 {
        let doc = Html::parse_document(&html);

        let now = date.format("%Y-%m-%d").to_string();
        let selector = format!("rect[data-date=\"{}\"]", now);

        let selector = Selector::parse(&selector).unwrap();
        let input = doc.select(&selector).next().unwrap();

        input
            .value()
            .attr("data-count")
            .unwrap()
            .parse()
            .unwrap_or_default()
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
                year_contributions: 0,
                month_contributions: 0,
                sum_contributions: 0
            }
        )
    }
    #[test]
    fn test_from_html() {
        let date = Local.ymd(2019, 1, 26);

        assert_eq!(
            ContriView::from_html(&sample_html(), date).unwrap_or_default(),
            ContriView {
                sum_contributions: 3532,
                month_contributions: 260,
                year_contributions: 260,
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
        assert_eq!(51, ContriView::week_contributions_from_html(&sample_html()))
    }

    #[test]
    fn test_today_contributions() {
        let date = Local.ymd(2019, 1, 26);

        assert_eq!(
            3,
            ContriView::today_contributions_from_html(&sample_html(), date)
        )
    }
    #[test]
    fn test_year_contributions() {
        let date = Local.ymd(2019, 1, 26);

        assert_eq!(
            260,
            ContriView::year_contributions_from_html(&sample_html(), date)
        )
    }

    #[test]
    fn test_month_contributions() {
        let date = Local.ymd(2019, 1, 26);

        assert_eq!(
            260,
            ContriView::month_contributions_from_html(&sample_html(), date)
        )
    }
}
