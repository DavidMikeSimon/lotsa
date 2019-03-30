use actix_web::{server, App, HttpRequest};

fn index(_req: &HttpRequest) -> &'static str {
  "Hello worlds!"
}

pub fn web_server() {
  server::new(|| App::new().resource("/", |r| r.f(index)))
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
}
