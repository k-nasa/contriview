mod contriview;

use crate::contriview::ContriView;
use reqwest::*;

fn main() {
    // clapにしても良いかもね
    let username = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please input username");
        std::process::exit(1);
    });
    let url = format!("https://github.com/users/{}/contributions", username);

    let client = Client::new();
    let mut resp = client.get(&url).send().unwrap();
    let html = resp.text().unwrap();

    println!("{:?}", ContriView::from_html(&html).unwrap())
}
