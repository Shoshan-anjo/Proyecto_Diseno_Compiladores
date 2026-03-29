use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Traductor")]
#[command(about = "Morse code to text translator", long_about = None)]
pub struct Args {
    /// Input morse code (space-separated or use default SOS)
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0.., default_value = "... --- ...")]
    pub input: Vec<String>,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}