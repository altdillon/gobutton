// https://doc.rust-lang.org/std/time/struct.Duration.html

use std::time::{Duration, Instant};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Play_button
{
    pressed: bool, // if button has been pressed
    time_pressed: Option<u64>,
    time_stopped: Option<u64>,
}


impl Play_button
{
    pub fn new() -> Play_button // like a constructor, but more like a factory
    {
        Play_button {pressed:false,time_pressed: None,time_stopped: None }
    }

    pub fn reg_press_play(&mut self) -> u64 // push the button
    {
        self.pressed = true;
        self.time_pressed = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()); // save the number of seconds since 1970
        self.time_pressed.unwrap()
    }

    pub fn get_delta_time(&mut self) -> u64
    {
        let dt = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - self.time_pressed.unwrap();

        self.pressed = false;
        self.time_pressed = None;

        dt // return dt
    }

    pub fn reg_press_stop(&mut self) -> u64
    {
        self.time_stopped=Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
        self.pressed = false;

        self.time_stopped.unwrap() // return the unix time
    }

    pub fn was_pressed(&self) -> bool
    {
        self.pressed
    }

}
