
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to model (multi-file models use the same prefix name and different extensions)
    pub model_file: String,


    /// Prompt input text
    pub prompt_text: String,
}
