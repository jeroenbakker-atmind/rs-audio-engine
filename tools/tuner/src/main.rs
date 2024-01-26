use arguments::Arguments;
use clap::Parser;
use tuner::Tuner;

mod arguments;
mod tuner;

fn main() {
    let arguments = Arguments::parse();
    let mut tuner = Tuner::new(arguments);
    tuner.start();
}
