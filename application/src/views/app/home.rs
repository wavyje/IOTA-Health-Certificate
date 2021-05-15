use actix_web::HttpResponse;
use super::content_loader::read_file;

pub async fn home() -> HttpResponse {
    let mut html_data = read_file("./templates/main.html");
    let css_data = read_file("./templates/css/main.css");

    html_data = html_data.replace("{{BASE_CSS}}", &css_data);
    
    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html_data)
}