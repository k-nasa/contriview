mod contriview;

use crate::contriview::ContriView;
use clap::*;
use reqwest::*;

fn main() {
    let matches = app().get_matches();

    let username = matches.value_of("username").unwrap();

    let url = format!("https://github.com/users/{}/contributions", username);

    let client = Client::new();
    let mut resp = client.get(&url).send().unwrap();
    let html = resp.text().unwrap();

    println!("{:?}", ContriView::from_html(&html).unwrap())
}

fn app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .arg(Arg::with_name("username").required(true))
}
