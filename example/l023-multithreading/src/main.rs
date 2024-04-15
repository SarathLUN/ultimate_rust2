use log::{info, error};
use std::thread;
use std::time::Duration;

fn sleep(secs: f32) {
    thread::sleep(Duration::from_secs_f32(secs));
}

pub mod dad {
    use super::*;

    pub fn cook_spaghetti() -> bool {
        info!("Cooking the spaghetti...");
        sleep(4.0);
        info!("Spaghetti is ready!");
        true
    }
}

pub mod mom{
    use super::*;
    pub fn cook_sauce(){
        info!("Cooking the sauce...");
        sleep(2.0);
        info!("sauce is ready!");
    }

    pub fn set_table(){
        info!("Setting up the table...");
        sleep(2.0);
        info!("Table is set!");
    }
}

fn main() {
    env_logger::init();
    let handle = thread::spawn(|| dad::cook_spaghetti());
    mom::cook_sauce();
    mom::set_table();
    if handle.join().unwrap_or(false) {
        info!("Spaghetti time! Yum!");
    }else {
        error!("Dad messed up the spaghetti. Order pizza instead?");
    }
}
