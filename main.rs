//basis of this try is the examples from sysfs_gpio crate here: https://crates.io/crates/sysfs_gpio



extern crate sysfs_gpio;

use std::env;
use sysfs_gpio::Pin;
use sysfs_gpio::Direction::High;
use std::time::Duration;
use std::thread::sleep;

use std::process::Command;

fn poll (pin: u64) -> sysfs_gpio::Result<()> {
    let input = Pin::new(pin);
    input.with_exported(|| {
       input.set_direction(High)?;    //set pin to High in case it starts at Low
    let mut prev_val: u8 = 255;

    loop {
        let val = input.get_value()?;   // get the value of the pin
        if val != prev_val {  //if the value changes then print the state of the pins
            println!("Pin State: {}", if val == 0 { "Low" } else { "High" });
            if val == 0 {
                Command::new("./pushtotext")   //if the pin value is low then execute rust app to send a text message
                .spawn()
                .expect("pushtotext command failed to start");
            }

            prev_val = val;
        }
        sleep(Duration::from_millis(1000));
            }
    })
}


fn main() {
let args: Vec<String> = env::args().collect(); 
loop { 
        println!("arg length: {}", args.len());

        match args[1].parse::<u64>() {
            Ok(pin) => match poll(pin) {
                Ok(()) => println!("polling complete!"),
                Err(err) => println!("Error: {}", err),
            },
            Err(_) => println!("Usage: ./interrupt <pin>MatchErr"),
            }
        }
    }