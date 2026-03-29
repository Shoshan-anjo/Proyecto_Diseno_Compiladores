use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Traductor")]
#[command(about = "Bidirectional Morse code translator", long_about = None)]
pub struct Args {
    /// Input text or morse code
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0.., default_value = "... --- ...")]
    pub input: Vec<String>,

    /// Translate to Morse code (default: Morse to text)
    #[arg(short = 't', long, help = "Translate text TO morse code")]
    pub to_morse: bool,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}