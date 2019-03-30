use actix_web::{server, App, fs};

pub fn main() {
  server::new(|| {
    App::new().handler("/", fs::StaticFiles::new("www").unwrap())
  })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
}
