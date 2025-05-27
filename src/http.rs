use crate::net::TcpStream;

pub struct HttpClient {
    stream: TcpStream,
    buffer: [u8; 1024],
}

impl HttpClient {
    pub fn new(server: &str, port: u16) -> Result<Self, &'static str> {
        let stream = TcpStream::connect(server, port)?;
        Ok(Self {
            stream,
            buffer: [0; 1024],
        })
    }

    pub fn get(&mut self, path: &str) -> Result<&str, &'static str> {
        let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, self.stream.host());
        self.stream.write(request.as_bytes())?;

        let len = self.stream.read(&mut self.buffer)?;
        Ok(unsafe { str::from_utf8_unchecked(&self.buffer[..len]) })
    }

    pub fn post_json(&mut self, path: &str, json: &str) -> Result<&str, &'static str> {
        let request = format!(
            "POST {} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            path,
            self.stream.host(),
            json.len(),
            json
        );
        self.stream.write(request.as_bytes())?;

        let len = self.stream.read(&mut self.buffer)?;
        Ok(unsafe { str::from_utf8_unchecked(&self.buffer[..len]) })
    }
}
