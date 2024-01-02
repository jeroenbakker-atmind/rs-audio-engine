use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    about = "Convert a (.wav) file to rust source",
    long_about = "Convert given <INPUT_FILENAME> to rust source file. The generated source code is printed to standard out. Multichannel wave files will be joined into a single channel. Resulting source file contains floats samples."
)]
pub struct Arguments {
    _app: String,

    /// Trim the start of the audio
    #[arg(long)]
    pub trim_start: bool,

    /// Normalize the sample to fit between -1 and 1
    #[arg(long)]
    pub normalize: bool,

    /// Input wave file to convert
    pub input_filename: String,
}
