
mod winapi;
use winapi::user32::*;

fn main() {
    MessageBox("Text", "Title", 1);
}
