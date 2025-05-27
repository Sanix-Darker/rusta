pub mod tcp;
pub mod ip;
pub mod eth;

pub struct TcpStream {
    socket: usize,
    host: &'static str,
}

impl TcpStream {
    pub fn connect(host: &str, port: u16) -> Result<Self, &'static str> {
        // TODO: low-level network initialization ...
        Ok(Self {
            socket: 0, // Would be actual socket ID
            host,
        })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        // TODO: send data ...
        Ok(data.len())
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, &'static str> {
        // TODO: receive data ...
        Ok(0) // Actual bytes read
    }

    pub fn host(&self) -> &str {
        self.host
    }
}
