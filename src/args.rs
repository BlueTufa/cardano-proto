use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about)]
pub struct Args {
    #[arg(short('p'), long, help = "The Cardano policy ID, as a string.")]
    pub policy_id: String,

    #[arg(short('d'), long, help = "The output directory for the assets.")]
    pub output_dir: String,
}
