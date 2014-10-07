extern crate time;

extern crate hyper;
extern crate url;

// use std::fmt;
use hyper::client::Request;
use url::Url;


fn main() {
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
