extern crate timer;
extern crate chrono;


pub mod gpio_access {
    pub struct GpioSystem{
        timer: timer::Timer
    }

    pub enum GpioState{
        HIGH,
        LOW
    }

    pub fn state_contrary(state: GpioState) -> GpioState{
        match state {
            GpioState::HIGH => return GpioState::LOW,
            GpioState::LOW => return GpioState::HIGH
        }
    }

    pub fn str_to_GpioState(state: &str) ->  std::result::Result<GpioState, &str>{
        match state {
            "HIGH" => return Result::Ok(GpioState::HIGH),
            "LOW" => return Result::Ok(GpioState::LOW),
            _ => return Result::Err("Invalid state type")
        }
    }

    pub fn init() -> GpioSystem {
        return GpioSystem{timer: timer::Timer::new()};
    }
    
    pub fn toggle_pin(gpio: &GpioSystem, pin: u8, time: Option<i64>, state: GpioState){
        match state {
            GpioState::HIGH => println!("set pin {} to HIGH", pin),
            GpioState::LOW => println!("set pin {} to LOW", pin)
        }

        match time {
            None => return,
            Some(duration) => {
                gpio.timer.schedule_with_delay(chrono::Duration::milliseconds(duration), move || {
                    match state {
                        GpioState::HIGH => println!("set pin {} to LOW", pin),
                        GpioState::LOW => println!("set pin {} to HIGH", pin)
                    }
                });
            }
        };
    }
}