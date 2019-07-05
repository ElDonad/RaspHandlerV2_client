#[macro_use]
extern crate json;


use std::net::TcpStream;
use std::io::Write;
use std::io;
use std::io::Read;
use std::fs;
mod gpio_access;

fn main() {

    let gpio = gpio_access::gpio_access::init();
    let data = fs::read_to_string("save.txt").expect("could not read file");
    let data: Vec<&str> = data.split('\n').collect();
    let auth = object!{
        "type" => "header",
        "headerType" => "authentificate",
        "id" => data[1]
    };


    let mut stream = TcpStream::connect(data[0]).expect("could not connect to server");

    stream.write((auth.dump() + "␄").as_bytes()).expect("could not send to server");
        
    stream.set_nonblocking(true).expect("could not set socket to non blocking");
    let mut message = "".to_owned();  
    loop{
        let mut buffer = vec![0; 2048];
        match stream.read(&mut buffer){
            Ok(_) => {
                message.push_str(std::str::from_utf8(&buffer).expect("could not convert to string"));
                if message.find('␄') != None{
                    break;
                }
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            },
            Err(ref e) => panic!("encountered io error : {}", e)
        };
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    let messages: Vec<&str> = message.split('␄').collect();
    for message in messages {
        print!("nouveau message : {}", message);
        let obj = json::parse(message).expect("invalid message");
        match obj["type"].as_str().expect("no message type found") {
            "authentificated" => {
                fs::write("save.txt", std::format!("{}\n{}",data[0], obj["id"].as_str().expect("no valid id provided")))
                .expect("writing to file failed");
                break;
                
            },
            "togglePin" => {
                println!("New pin switch requested ! ");
                let pin_number = obj["pin"].as_u8().expect("invalid pin number");
                let duration = if obj.is_null() {None} 
                    else {Some(obj["duration"].as_i64().expect("invalid duration value"))};
                let state = gpio_access::gpio_access::str_to_GpioState(
                    obj["state"].as_str().expect("invalid state string")
                ).expect("");

                gpio_access::gpio_access::toggle_pin(&gpio, pin_number, duration, state);
            }

            _=>panic!("Uncovered message type")
        }
    }

}
