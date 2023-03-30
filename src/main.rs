#[macro_use] extern crate rocket; 
use rocket::tokio::time::{sleep, Duration}; 
use rocket::serde::{Serialize, json::Json}; 

#[derive(Serialize)]
#[serde(crate="rocket::serde")]
struct Task {
    done: bool
}

#[get("/todo")]
fn todo() -> Json<Task> {
    Json(Task {
        done: false
    })
}
#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await; 
    format!("Waited for {} seconds", seconds)
}
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, delay, todo])
}
