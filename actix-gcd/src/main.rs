use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use validator::Validate;

fn main() {
    let server = HttpServer::new( || {
        App::new().route("/", web::get().to(get_index))
                  .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:3000...");
    server.bind("127.0.0.1:3000").expect("error binding server to address")
        .run().expect("error running server");
    
}

fn get_index() -> HttpResponse {
    return HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
              <title>GCD Calculation</title>
              <form action="/gcd" method="post">
                <input type="text" name="n" />
                <input type="text" name="m" />
                <button type="submit">Compute GCD</button>
              </form>  
            "#,
            );
}

#[derive(Deserialize,Validate)]
struct GcdParameters {
  #[validate(range(min = 0))]
  n : u64,
  #[validate(range(min = 0))]
  m : u64,
}

fn post_gcd( form: web::Form<GcdParameters>) -> HttpResponse {
  if form.n == 0 || form.m == 0 {
    return HttpResponse::BadRequest()
      .content_type("text/html")
      .body("Computing the GCD with zero is boring");
  }

  let response =
    format!("The greatest common divisor of the numbers {} an {} \
            is <b>{}</b>\n",
            form.n, form.m, gcd(form.m, form.n));

  return HttpResponse::Ok()
    .content_type("text/html")
    .body(response);
}


fn gcd(mut m: u64, mut n: u64) -> u64 {
  assert!(m != 0 && n != 0);
  while m != 0 {
      if m < n {
          let temp = m;
          m = n;
          n = temp;
      }
      m = m % n;
  }
   return n;
}