mod theme;

use crate::theme::Theme;
use clap::{crate_name, crate_version};
use tracing::Level;

fn main() {
    let ansi_enabled = fix_ansi_term();

    tracing_subscriber::fmt()
        .with_ansi(ansi_enabled)
        .with_max_level(Level::DEBUG)
        .init();

    let matches = clap::Command::new(crate_name!())
        .version(crate_version!())
        .author("Chris Lieb")
        .about("Generate beautiful map posters for any city")
        .subcommand(clap::Command::new("list-themes").about("List all available themes"))
        .subcommand(
            clap::Command::new("generate")
                .alias("gen")
                .about("Generate a map poster")
                .arg(
                    clap::Arg::new("city")
                        .long("city")
                        .short('c')
                        .required(true)
                        .num_args(1)
                        .help("City name"),
                )
                .arg(
                    clap::Arg::new("country")
                        .long("country")
                        .short('C')
                        .required(true)
                        .num_args(1)
                        .help("Country name"),
                )
                .arg(
                    clap::Arg::new("theme")
                        .long("theme")
                        .short('t')
                        .required(false)
                        .num_args(1)
                        .default_value("feature_based")
                        .help("Theme name"),
                )
                .arg(
                    clap::Arg::new("distance")
                        .long("distance")
                        .short('d')
                        .required(false)
                        .num_args(1)
                        .default_value("29000")
                        .value_parser(clap::value_parser!(u16))
                        .help("Map radius in meters"),
                ),
        )
        .subcommand_required(true)
        .get_matches();

    match matches.subcommand() {
        Some(("list-themes", _)) => list_themes(),
        Some((x, _)) => panic!("Unknown subcommand: {}", x),
        None => panic!("No subcommand specified!"),
    }
}

#[cfg(target_os = "windows")]
fn fix_ansi_term() -> bool {
    nu_ansi_term::enable_ansi_support().is_ok_and(|()| true)
}

#[cfg(not(target_os = "windows"))]
fn fix_ansi_term() -> bool {
    true
}

/// List all available themes with descriptions.
pub fn list_themes() {
    let mut themes = Theme::get_available_names();
    if themes.is_empty() {
        panic!("No themes found!");
    }

    println!("Available themes:");
    println!("{}", "-".repeat(60));
    themes.sort();
    themes
        .iter()
        .map(|name| match Theme::get_by_name(name) {
            Ok(theme) => (name, theme.name, Some(theme.description)),
            Err(_) => (name, name.clone(), None),
        })
        .for_each(|(name, display_name, description)| {
            println!("  {} ({})", display_name, name);
            match description {
                Some(desc) => println!("    {}", desc),
                None => (),
            }
        });
}
