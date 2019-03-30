use actix_web::{server, App, fs};

pub fn main() {
  server::new(|| {
    App::new().handler("/", fs::StaticFiles::new("www/dist").unwrap().index_file("index.html"))
  })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
}
