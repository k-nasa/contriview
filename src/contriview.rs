use chrono::*;
use failure::Error;
use scraper::*;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ContriView {
    today_contributions: u32,
    week_contributions: u32,
    month_contributions: u32,
    year_contributions: u32,
    week_ave: u32,
    month_ave: u32,
    sum_ave: u32,
    sum_contributions: u32,
}

impl std::fmt::Display for ContriView {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "today_contributions: {}
week_contributions: {}
month_contributions: {}
year_contributions: {}
sum_contributions: {}
week_ave: {}
month_ave: {}
sum_ave: {}",
            self.today_contributions,
            self.week_contributions,
            self.month_contributions,
            self.year_contributions,
            self.sum_contributions,
            self.week_ave,
            self.month_ave,
            self.sum_ave
        )
    }
}

impl ContriView {
    pub fn from_html(html: &str, date: Date<Local>) -> Result<Self, Error> {
        let sum_contributions = Self::sum_contributions_from_html(html);
        let week_contributions = Self::week_contributions_from_html(html);
        let year_contributions = Self::year_contributions_from_html(html, date);
        let month_contributions = Self::month_contributions_from_html(html, date);
        let today_contributions = Self::today_contributions_from_html(html, date);
        let week_ave = week_contributions / 7;
        let month_ave = month_contributions / date.day();
        let sum_ave = sum_contributions / 365;

        Ok(ContriView {
            sum_contributions,
            week_contributions,
            month_contributions,
            year_contributions,
            today_contributions,
            week_ave,
            month_ave,
            sum_ave,
        })
    }

    fn sum_contributions_from_html(html: &str) -> u32 {
        let doc = Html::parse_document(&html);
        let selector = Selector::parse(r#"rect[data-date]"#).unwrap();
        let input = doc.select(&selector);

        input
            .map(|i| -> u32 {
                let contribution = i.value().attr("data-count").unwrap();
                contribution.parse().unwrap_or_default()
            })
            .sum()
    }

    fn month_contributions_from_html(html: &str, date: Date<Local>) -> u32 {
        let doc = Html::parse_document(&html);

        let now = date.format("%Y-%m").to_string();
        let selector = format!("rect[data-date^=\"{}\"]", now);

        let selector = Selector::parse(&selector).unwrap();
        let input = doc.select(&selector);

        let contributions: Vec<u32> = input
            .map(|i| -> u32 {
                let contribution = i.value().attr("data-count").unwrap();
                contribution.parse().unwrap_or_default()
            })
            .collect();

        contributions.iter().sum()
    }

    fn week_contributions_from_html(html: &str) -> u32 {
        let doc = Html::parse_document(&html);
        let selector = Selector::parse(r#"rect[data-date]"#).unwrap();
        let input = doc.select(&selector);

        let contributions: Vec<u32> = input
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
            .map(|i| -> u32 {
                let contribution = i.value().attr("data-count").unwrap();
                contribution.parse().unwrap_or_default()
            })
            .collect();

        contributions.iter().sum()
    }

    fn today_contributions_from_html(html: &str, date: Date<Local>) -> u32 {
        let doc = Html::parse_document(&html);

        let now = date.format("%Y-%m-%d").to_string();
        let selector = format!("rect[data-date=\"{}\"]", now);

        let selector = Selector::parse(&selector).unwrap();
        let input = doc.select(&selector).next();

        if input.is_none() {
            return 0;
        }

        input
            .unwrap()
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
                sum_contributions: 0,
                week_ave: 0,
                month_ave: 0,
                sum_ave: 0,
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
                today_contributions: 3,
                week_ave: 7,
                month_ave: 10,
                sum_ave: 9,
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
