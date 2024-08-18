
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {

    /// Constants like "client" and "server" to indicate how rest of args are to be used
    pub command: String,

    /// Path to model (multi-file models use the same prefix name and different extensions)
    pub model_file: String,


    /// Prompt input text
    pub prompt_text: String,
}
