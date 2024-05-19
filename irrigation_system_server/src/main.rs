use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use rocket::fs::{relative, FileServer};
use rocket::response::Redirect;
use rocket::tokio::sync::oneshot;

static mut ROUTINE_STATUS: Lazy<Arc<Mutex<Option<(oneshot::Sender<u32>, oneshot::Receiver<u32>)>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

#[cfg(test)]
mod tests;

#[macro_use]
extern crate rocket;

#[derive(FromForm)]
struct Routine {
    routine: Vec<Zone>,
}

#[derive(FromForm)]
struct Zone {
    zone: Zones,
    #[field(validate = range(..3600))]
    duration: u16,
}

#[derive(FromFormField)]
enum Zones {
    #[field(value = "f")]
    Front,
    #[field(value = "bp")]
    BackPool,
    #[field(value = "bs")]
    BackShed,
}

#[get("/?<routine..>")]
fn run(routine: Option<Routine>) -> String {
    let mut response = String::new();

    match routine {
        Some(_routine) => {
            if _routine.routine.len() != 3 {
                response.push_str("Zones number is not three\n");
            }

            let all_zones = (1 << Zones::Front as u8)
                | (1 << Zones::BackShed as u8)
                | (1 << Zones::BackPool as u8);

            let mut vec_mask = 0;

            for zone in _routine.routine {
                let zone_code = zone.zone as u8;
                response.push_str(&format!(
                    "Zone: {}, Duration: {}\n",
                    zone_code, zone.duration
                ));
                vec_mask |= 1 << zone_code;
            }

            if vec_mask != all_zones {
                response.push_str("Missing zones in routine, bitmask: ");
                response.push_str(&format!("{vec_mask}"));
                response.push_str("\n");
            }
        }
        None => response.push_str("Malformed request"),
    }

    response
}

#[get("/status")]
fn status() -> String {
    let mut response = String::new();

    unsafe {
        let routine_status = Arc::clone(&ROUTINE_STATUS);
        let opt = routine_status.lock().unwrap();

        match *opt {
            Some(_) => {
                response.push_str("Current operation");}
            None => {
                response.push_str("Not currently running");}
        }
    }

    response
}

#[get("/stop")]
fn stop() -> String {
    let mut response = String::new();

    response.push_str("Stopped current routine");

    response
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/prod"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/routine", routes![stop, status])
        .mount("/routine/run", routes![run])
        .mount("/", routes![index])
        .mount("/prod/", FileServer::from(relative!("static")))
}
