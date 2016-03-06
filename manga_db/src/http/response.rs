
use std::collections::HashMap;
use std::io::{ Read, Write, BufReader, BufWriter, Error };
use std::net::TcpStream;

#[derive(Debug)]
pub struct Response<'a> {
    url: &'a str,
    status: String,
    headers: HashMap<String, String>,
    content: String,
}

impl<'a> Response<'a> {
    pub fn new<T: AsRef<str> + 'a>(url: &T) -> Result<Response<'a>, Error> {
        Ok(Response {
            url: url.as_ref(),
            status: String::new(),
            headers: HashMap::new(),
            content: String::new(),
        })
    }

    pub fn get_response<T: AsRef<str>>(&mut self, req: &T) -> Result<Response<'a>, Error> {
        let mut stream = try!(TcpStream::connect(self.url));

        let mut writer = BufWriter::new(stream);
        writer.write(req.as_ref().as_bytes());

        let mut reader = BufReader::new(try!(writer.into_inner()));
        let mut page = String::new();
        reader.read_to_string(&mut page);

        println!("Page: {}", page);

        Ok(Response {
            url: "",
            status: String::new(),
            headers: HashMap::new(),
            content: String::new(),
        })
    }
}
