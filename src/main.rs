use rppal::gpio::Gpio;
use std::error::Error;
use std::time::Duration;
use std::thread;
use rppal::system::DeviceInfo;
use std::process::Command;
use cmd_lib::{info, warn, run_cmd, run_fun, CmdResult, FunResult};

const SHUTDOWN_PIN: u8 = 4;
fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world! on a {}", DeviceInfo::new()?.model());
    let mut pin = Gpio::new()?.get(SHUTDOWN_PIN)?.into_input_pullup();
    let mut run = true;
    while run {
      let state = pin.is_low();
      println!("state is low? 	{}", state);
      thread::sleep(Duration::from_millis(2000));
      println!("again");
      if (state){
        println!("shutting down");
        //Command::new("shutdown").arg("-h").arg("now").spawn().expect("failed");
        let res = run_cmd!("sudo shutdown -h now");
        println!("result: {:?}", res);
        thread::sleep(Duration::from_millis(5000));
        run = false;        
      }
    }
    Ok(())
}
