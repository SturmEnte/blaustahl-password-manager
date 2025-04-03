use std::time::Duration;
use serialport;
pub struct BlaustahlSRWP {
    port: String,
}

impl BlaustahlSRWP {
    pub fn new(port: String) -> BlaustahlSRWP {

        let port = serialport::new("COM3", 115_200);

        return BlaustahlSRWP { port: port }
    }

    pub fn connect(port: String) -> Result<(), String> {
        Ok(())
    }
}