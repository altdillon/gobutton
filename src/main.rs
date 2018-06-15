extern crate gobutton;
// import stuff for wring pi
extern crate wiringpi;
extern crate serde_json; // for json encoding


//extern crate serde;
//extern crate serde_json;

#[macro_use] extern crate serde_derive;

use serde_json::Error;

use wiringpi::pin::Value::{High, Low};
use std::{thread, time};

// include the stuff for button pushing
use gobutton::button::Play_button;
// include the stuff for using the network
use std::net::*; // all the stuff for using the network



fn multicast_setup() -> UdpSocket
{
    // new up the address and interfaces
    let multicast_address = Ipv4Addr::new(224,3,29,71); //multicast_group = '224.3.29.71'
    let interface = Ipv4Addr::new(0,0,0,0); // I don't fully know how this works and it may caouse some issues in the future
    // setup a new udp packet struct
    let socket_result = UdpSocket::bind("192.168.1.17:5006");

    match socket_result
    {
        Ok(UdpSocket) => UdpSocket,
        Err(error) => {
            panic!("Something went wrong while attmepting to bind the socket: {:?}",error);
        },
    }

}


enum button_command
{
    start,
    stop,
}

#[derive(Serialize, Deserialize)]
struct ButtonTemplate
{
    command: String,
    timestamp: u64,
}

// take in a command and time and return a json formatted string
fn setupCommand (command:button_command,time_stamp: u64) -> String
{
    // use the json macro from serde to setup the json string

    // if let button_command::start = command // start command
    // {
    //     println!("start");
    //     let command
    // }
    // else if let button_command::stop = command // stop command
    // {
    //     println!("stop");
    // }


    // "hello"

    let cmd = ButtonTemplate {command: if let button_command::start = command {String::from("start")} else {String::from("stop")}, timestamp: time_stamp };
    serde_json::to_string(&cmd).unwrap()
}

fn main()
{
    // let st=setupCommand(button_command::stop,18446744073709551615);
    // println!("{}",st);

    let mut pbutton = Play_button::new(); // setup a new struct for the play Play_button
    let wpi = wiringpi::setup();
    let start_button = wpi.input_pin(21); // gpio pin #5, attached to the green button
    let stop_button = wpi.input_pin(22); // gpio pin #6, attached to the red button.  Becouse it's a stop button... but it's not a quitter
    let indicator_led = wpi.output_pin(23);

    // setup the multicast socket
    let multicast_address = Ipv4Addr::new(224,3,29,71); //multicast_group = '224.3.29.71'
    let interface = Ipv4Addr::new(0,0,0,0); // I don't fully know how this works and it may caouse some issues in the future
    let mut socket = multicast_setup();
    socket.join_multicast_v4(&multicast_address,&interface);
    socket.set_broadcast(true);

    loop
    {
        if start_button.digital_read() == Low && pbutton.was_pressed() == false  // button is active low
        {
            let start_time = pbutton.reg_press_play();
            // let mut command = String::from("\"command\":");
            // command.push_str("\"playAt\",");
            // command.push_str("\"time:\"");
            // command.push_str(start_time.to_string().as_str());
            // //println!("{}",command);
            // let message = command.as_bytes(); // needs to be byte data to send it to multicast netowrk
            let command = setupCommand(button_command::start,start_time);
            let message = command.as_bytes();
            let success = socket.send_to(&message[0..message.len()],"224.3.29.71:5005");
            indicator_led.digital_write(High); // play indicator goes high
        }

        if stop_button.digital_read() == Low && pbutton.was_pressed() == true
        {
            let stoptime = pbutton.reg_press_stop();
            // let mut command = String::from("\"command\":");
            // command.push_str("\"stop\",");
            // command.push_str("\"delta_time:\"");
            // command.push_str(deltaTime.to_string().as_str());
            //println!("{}",command);
            let command = setupCommand(button_command::stop,stoptime);
            let message = command.as_bytes(); // needs to be byte data to send it to multicast netowrk
            let success = socket.send_to(&message[0..message.len()],"224.3.29.71:5005");
            indicator_led.digital_write(Low); // play indicator goes low
        }
    }
}
