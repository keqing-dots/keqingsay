use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "keqingsay")]
#[command(
    about = "Keqingsay is a CLI program like cowsay, but instead of a talking cow, it's Keqing from Genshin Impact"
)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// The text for Keqing to say (used when no subcommand is given)
    pub text: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Display an animated Keqing (variant 1, 2, or 3, default: 1)
    Animate {
        /// The text for Keqing to say
        text: Option<String>,
        /// Animation variant (1, 2, or 3)
        #[arg(short, long, default_value = "1")]
        variant: u8,
    },

    /// Display Keqing in freestyle mode, cycling through all variants
    Freestyle {
        /// The text for Keqing to say
        text: Option<String>,
    },
}
