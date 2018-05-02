extern crate shio;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use shio::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "hello-heroku", about = "Hello, Heroku!")]
struct Options {
  #[structopt(short = "p", long = "port", default_value = "7878")]
  port: u16,
}

fn hello_heroku(_: Context) -> Response {
  Response::with("Hello, Heroku!")
}

fn main() {
  let options = Options::from_args();

  Shio::default()
    .route((Method::Get, "/hello-heroku", hello_heroku))
    .run(format!(":{}", options.port))
    .unwrap()
}
