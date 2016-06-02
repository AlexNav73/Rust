
use super::rs;

pub struct SimpleTestObject {
    a: i32,
    b: u16
}

impl rs::Encodable for SimpleTestObject {
    fn encode<S: rs::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        try!(s.emit_i32(self.a));
        s.emit_u16(self.b)
    }
}

impl rs::Decodable for SimpleTestObject {
    fn decode<D: rs::Decoder>(d: &mut D) -> Result<Self, D::Error> {
        Ok(SimpleTestObject {
            a: try!(d.read_i32()),
            b: try!(d.read_u16())
        })
    }
}
