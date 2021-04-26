#[macro_use]
extern crate prettytable;
extern crate reqwest;


use prettytable::{Table};
use select::document::Document;
use select::predicate::{Class};

fn main() {
    let url = "https://quotes.toscrape.com/";

    let resp = reqwest::blocking::get(url).unwrap();

    let document = Document::from_read(resp).unwrap();

    let mut table = Table::new();

    for data in document.find(Class("quote")) {
        let title = data.find(Class("text")).next().unwrap();

        let author = data.find(Class("author")).next().unwrap();

        let list_quotes = format!("{} | {}", title.text(), author.text());

        table.add_row(row![FdBybl->list_quotes]);
        // table.add_row(row![Fy->author]);
    }
    table.printstd();
}
