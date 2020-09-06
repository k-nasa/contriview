mod contriview;

use crate::contriview::ContriView;
use chrono::*;
use clap::*;
use reqwest::blocking::Client;

fn main() {
    let matches = app().get_matches();

    let username = match matches.value_of("username") {
        None => {
            app().print_help().unwrap();
            return;
        }
        Some(u) => u,
    };

    let url = format!("https://github.com/users/{}/contributions", username);

    let client = Client::new();
    let resp = match client.get(&url).send() {
        Ok(res) => res,
        Err(_) => {
            println!("Failed fetch from {}", url);
            return;
        }
    };

    let html = resp.text().unwrap();

    let date = match matches.value_of("date") {
        Some(d) => date_from_string(d),
        None => Local::today(),
    };

    println!("{}", ContriView::from_html(&html, date).unwrap())
}

fn app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .arg(Arg::with_name("username").required(true))
        .arg(
            Arg::with_name("date")
                .help("date")
                .value_name("date")
                .short("d"),
        )
}

fn date_from_string(date: &str) -> Date<Local> {
    let v: Vec<u32> = date.split('-').map(|v| v.parse::<u32>().unwrap()).collect();

    Local.ymd(v[0] as i32, v[1], v[2])
}
