
use structopt::StructOpt;
use std::io::{BufReader, BufRead};
use std::fs::File;
use chrono::{Datelike, NaiveDate, Weekday, Date};
use std::borrow::BorrowMut;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn getPennyArcadeDates() -> Vec<NaiveDate> {
    /*
        Penny Arcade started publishing comics every Mon, Wed, Fri on Nov 18, 1998.

        Returns a vector of dates that should have a comic for us to go download.
     */
    let mut dates: Vec<NaiveDate> = Vec::new();

    let today = chrono::Utc::today().naive_utc();

    let comicDays = [chrono::Weekday::Mon, chrono::Weekday::Wed, chrono::Weekday::Fri];
    for (idx, day) in NaiveDate::from_ymd(1998, 11, 18).iter_days().filter(|d| comicDays.contains(&d.weekday())).enumerate() {
        if day > today {
            break;
        }
        println!("{}: comic to download on {}", idx, day);
        dates.push(day);
    }

    dates
}



fn downloadHTML() {
    let start_day = time::strptime("1998-11-18", "%Y-%m-%d").ok().unwrap();
    println!("start day: {}", start_day);

    let today = time::now();

    println!("today: {}", today);

    let url = format!("{}/{}", "http://www.penny-arcade.com/comic", start_day.strftime("%Y/%m/%d"));

    println!("{}", Url::parse(url.as_slice()));
    // http://penny-arcade.com/comic/1998/11/18/the-sin-of-long-load-times
    // http://art.penny-arcade.com/photos/214584757_tSa5c-L-2.jpg

    let mut req = Request::get(
        Url::parse(url.as_slice()).unwrap()
    ).ok().unwrap();
    let mut res = req.start().ok().unwrap()
        .send().ok().unwrap();

    let status = res.status;
    println!("Response code: {}", status);
    println!("Headers: {}", res.headers);

    let body = res.read_to_string().unwrap();
    println!("Response: {}", body);
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let dates = getPennyArcadeDates();
    println!("Length of dates list: {}", dates.len());

    downloadHTML();

    println!("pattern: {}",args.pattern);
    println!("path: {:#?}", args.path);
    let infile :File = match File::open(&args.path) {
        Ok(file) => { file },
        Err(error) => { return Err(error.into()); }
    };

    let reader = BufReader::new(infile);
    for line in reader.lines().map(|l| l.unwrap()) {
        if line.contains(&args.pattern) {
            println!("match: {}", line);
        }
    }
    Ok(())
}
