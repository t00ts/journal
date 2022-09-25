use std::fs::OpenOptions;
use std::io::Write;

use chrono_tz::America::New_York;
use dialoguer::{theme::ColorfulTheme, Input};
use chrono::{Utc, TimeZone};

fn main() {

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("journal.log")
        .unwrap();

    let utc = Utc::now().naive_utc();
    let nyt = New_York.from_utc_datetime(&utc);

    if let Err(e) = writeln!(file, "----- {} ------", nyt.format("%d/%m/%Y %H:%M")) {
        eprintln!("Couldn't write to file: {}", e);
    }

    loop {

        let input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(">")
            .interact_text()
            .unwrap();

        let utc = Utc::now().naive_utc();
        let nyt = New_York.from_utc_datetime(&utc).format("%H:%M:%S");

        if let Err(e) = writeln!(file, "{} > {}", nyt, input) {
            eprintln!("Couldn't write to file: {}", e);
        }

    }
}
