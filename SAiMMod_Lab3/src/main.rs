
mod modules;

use modules::source::Source;
use modules::consumer::Consumer;

fn main() {

    let mut source = Source::new();
    let consumer = Consumer::new();

    source.set_next(consumer);
}
