mod crypto;
mod drone_comm;
mod error;
mod fec;
mod fhss;
mod packet;
mod interference;

use crate::drone_comm::DroneComm;
use crate::error::Result;

fn main() -> Result<()> {
    let mut drone_comm = DroneComm::new()?;

    let message = b"Hello, drone swarm!";
    drone_comm.send_message(message)?;

    match drone_comm.receive_message() {
        Ok(received) => println!("Received message: {:?}", received),
        Err(e) => eprintln!("Error receiving message: {}", e),
    }

    Ok(())
}
