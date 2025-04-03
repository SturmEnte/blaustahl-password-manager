use std::time::Duration;

use serialport;

pub struct BlaustahlSRWP {
    port: String,
}

impl BlaustahlSRWP {
    pub fn new(path: String) -> BlaustahlSRWP {

        let mut port = serialport::new(&path, 115_200).timeout(Duration::from_millis(10)).open().expect("Failed to open port");

        // Write
        let data: [u8; 21] = [
            0x00, 
            0x02, 
            0x00, 0x00, 0x00, 0x00, 
            0x0B, 0x00, 0x00, 0x00, 
            0x48, 0x45, 0x4C, 0x4C, 0x4F, 0x20, 0x57, 0x4F, 0x52, 0x4C, 0x44];

        match port.write(&data) {
            Ok(_) => println!("Written data"),
            Err(e) => eprintln!("Error while writing the data: {}", e),
        }

        std::mem::drop(data);

        port.flush().unwrap();
        
        // Read
        let data: [u8; 10] = [
            0x00, 
            0x01, 
            0x00, 0x00, 0x00, 0x00, 
            0x0B, 0x00, 0x00, 0x00];

        match port.write(&data) {
            Ok(_) => println!("Written data"),
            Err(e) => eprintln!("Error while writing the data: {}", e),
        }

        port.flush().unwrap();

        let mut buffer = [0u8; 11];
        let mut total_bytes_read = 0;

        while total_bytes_read < buffer.len() {
            match port.read(&mut buffer[total_bytes_read..]) {
                Ok(bytes_read) => {
                    total_bytes_read += bytes_read;
                    if bytes_read == 0 {
                        eprintln!("No more data available to read.");
                        break;
                    }
                }
                Err(e) => {
                    println!("Error while reading the data: {}", e);
                    break;
                }
            }
        }

        println!("Read {} bytes: {:?}", total_bytes_read, &buffer[..total_bytes_read]);

        return BlaustahlSRWP { port: path }
    }

    pub fn connect(port: String) -> Result<(), String> {
        Ok(())
    }
}