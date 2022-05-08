#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::io::Read;

#[get("/<length>")]
fn nyaa(length: u16) -> String {
    let mut file = std::fs::File::open("/dev/uwurandom").unwrap();
    let mut buf = vec![0; length as usize];
    file.read_exact(&mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}

fn main() {
    rocket::ignite().mount("/nya", routes![nyaa]).launch();
}
