#[macro_use]
extern crate rocket;

mod forecast;

use forecast::{Forecast, ForecastRequest, ForecastResponse};
use rocket::{fs::FileServer, serde::json::Json};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("www/static"))
        .mount("/api", routes![get_forecast])
}

#[get("/forecast?<request..>")]
fn get_forecast(request: ForecastRequest) -> Json<ForecastResponse> {
    let forecast = Forecast::from(request);

    forecast.response()
}
