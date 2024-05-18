#[cfg(test)] mod tests;

#[macro_use] extern crate rocket;

#[derive(FromForm)]
struct Routine {
    routine: Vec<Zone>
}


#[derive(FromForm)]
struct Zone {
    zone: Zones,
    #[field(validate = range(..3600))]
    duration: u16
}

#[derive(FromFormField, Clone)]
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

            let all_zones   = (1 << Zones::Front as u8)
            | (1 << Zones::BackShed as u8)
            | (1 << Zones::BackPool as u8);
    
            let mut vec_mask = 0;
            
            let mut total_duration = 0;
            
            for zone in _routine.routine.iter() {
                let zone_code = zone.zone.clone() as u8;
                response.push_str(&format!("Zone: {}, Duration: {}\n", zone_code, zone.duration));
                vec_mask |= 1 << zone_code;

                total_duration += zone.duration;
            }

            if vec_mask != all_zones {
                response.push_str("Missing zones in routine, bitmask: ");
                response.push_str(&format!("{vec_mask}"));
                response.push_str("\n");
            }

            match hardware::start(total_duration) {
                Err(_) => {response.push_str("Routine already started")}
                Ok(_) => {
                    let mut accumulated_delay = 0;
                    for zone in _routine.routine.iter() {
                        match zone.zone {
                            Zones::Front => {hardware::run_routine(14, accumulated_delay, zone.duration)}
                            Zones::BackPool =>{hardware::run_routine(15, accumulated_delay, zone.duration)}
                            Zones::BackShed => {hardware::run_routine(18, accumulated_delay, zone.duration)}
                        }
        
                        accumulated_delay += zone.duration;
                    }
                }
            }
        }
        None => response.push_str("Malformed request")
    }

    response
}

#[get("/status")]
fn status() -> String {
    let mut response = String::new();

    response.push_str("Status");

    response
}

#[get("/stop")]
fn stop() -> String {
    let mut response = String::new();

    hardware::stop();
    response.push_str("Stopped current routine");

    response
}

mod hardware {
    use rocket::tokio::spawn;
    use rocket::tokio::time::{Duration, Instant, sleep};
    use std::sync::atomic::{AtomicBool, Ordering};
    static RUN_FLAG: AtomicBool = AtomicBool::new(false);

    pub fn run_routine(pin: u8, delay:u16, duration:u16) {
        spawn(_run_routine(pin, delay, duration));
    }

    async fn _run_routine(pin: u8, delay: u16, duration: u16) {
        atomic_wait(delay as u64).await;
        if RUN_FLAG.load(Ordering::Relaxed) {
            set_pin(pin, true);
        }
        atomic_wait(duration as u64).await;
        set_pin(pin, false);
    }

    pub fn start(total_duration: u16) -> Result<(), ()> {
        if RUN_FLAG.load(Ordering::Relaxed) {
            return Err(())
        }

        spawn(_start(total_duration));

        Ok(())
    }

    async fn _start(total_duration: u16){
        RUN_FLAG.store(true, Ordering::SeqCst);

        atomic_wait(total_duration as u64).await;
        _stop().await;
    }

    pub fn stop() {
        spawn(_stop());
    }

    async fn _stop() {
        RUN_FLAG.store(false, Ordering::SeqCst);
    }

    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    fn set_pin(pin: u8, state: bool) {

    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn set_pin(pin: u8, state: bool) {
        println!("Pin: {pin}, State: {state}")
    }

    async fn atomic_wait(duration: u64) {
        if duration <= 0 {
            return;
        }

        let start = Instant::now();

        while RUN_FLAG.load(Ordering::Relaxed) {
            sleep(Duration::from_secs(1)).await;

            let now = Instant::now();
            let elapsed = now.checked_duration_since(start).expect("");

            if elapsed.as_secs() > (duration as u64) {
                break;
            }
        }
    }
}

use rocket::response::Redirect;
use rocket::fs::{FileServer, relative};

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
