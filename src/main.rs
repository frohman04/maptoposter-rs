mod location;
mod theme;

use crate::location::Location;
use crate::theme::Theme;
use clap::{crate_name, crate_version};
use std::path::PathBuf;
use std::time::SystemTime;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use tracing::{Level, info};

static THEME_DIR: &str = "themes";

fn main() {
    let ansi_enabled = fix_ansi_term();

    tracing_subscriber::fmt()
        .with_ansi(ansi_enabled)
        .with_max_level(Level::INFO)
        .init();

    let matches = clap::Command::new(crate_name!())
        .version(crate_version!())
        .author("Chris Lieb")
        .about("Generate beautiful map posters for any city")
        .subcommand(
            clap::Command::new("list-themes")
                .about("List all available themes")
                .arg(
                    clap::Arg::new("theme-dir")
                        .long("theme-dir")
                        .required(false)
                        .num_args(1)
                        .default_value(THEME_DIR)
                        .help("The path to the directory with the theme .json files"),
                ),
        )
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
                    clap::Arg::new("state")
                        .long("state")
                        .short('s')
                        .required(false)
                        .num_args(1)
                        .help("Optional state/province name"),
                )
                .arg(
                    clap::Arg::new("postal-code")
                        .long("postal-code")
                        .short('p')
                        .required(false)
                        .num_args(1)
                        .help("Optional postal code"),
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
                )
                .arg(
                    clap::Arg::new("theme-dir")
                        .long("theme-dir")
                        .required(false)
                        .num_args(1)
                        .default_value(THEME_DIR)
                        .help("The path to the directory with the theme .json files"),
                )
                .arg(
                    clap::Arg::new("font-dir")
                        .long("font-dir")
                        .required(false)
                        .num_args(1)
                        .default_value("fonts")
                        .help("The path to the directory with the fonts to use on the posters"),
                )
                .arg(
                    clap::Arg::new("output-dir")
                        .short('o')
                        .long("output-dir")
                        .required(false)
                        .num_args(1)
                        .default_value("posters")
                        .help("The path to the directory to output the posters to"),
                ),
        )
        .subcommand_required(true)
        .get_matches();

    match matches.subcommand() {
        Some(("list-themes", sub_match)) => {
            list_themes(sub_match.get_one::<String>("theme-dir").unwrap())
        }
        Some(("generate", sub_match)) => create_poster(
            sub_match.get_one::<String>("city").unwrap(),
            sub_match.get_one::<String>("country").unwrap(),
            sub_match.get_one::<String>("state").cloned(),
            sub_match.get_one::<String>("postal-code").cloned(),
            sub_match.get_one::<String>("theme").unwrap(),
            sub_match.get_one::<String>("theme-dir").unwrap(),
            *sub_match.get_one::<u16>("distance").unwrap(),
        ),
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
pub fn list_themes(theme_dir: &str) {
    let mut themes = Theme::get_available_names(theme_dir);
    if themes.is_empty() {
        panic!("No themes found!");
    }

    println!("Available themes:");
    println!("{}", "-".repeat(60));
    themes.sort();
    themes
        .iter()
        .map(|name| match Theme::get_by_name(name, theme_dir) {
            Ok(theme) => (name, theme.name, Some(theme.description)),
            Err(_) => (name, name.clone(), None),
        })
        .for_each(|(name, display_name, description)| {
            println!("  {} ({})", display_name, name);
            if let Some(d) = description {
                println!("    {}", d);
            }
        });
}

pub fn create_poster(
    city: &str,
    country: &str,
    state: Option<String>,
    postal_code: Option<String>,
    theme_name: &str,
    theme_dir: &str,
    _distance: u16,
) {
    let location = Location::from_name(city, country, &state, &postal_code);
    info!("✓ Found: {}", location.display_name);
    info!("✓ Coordinates: {}, {}", location.lat, location.lon);

    let _output_file = generate_output_filename(city, theme_name, theme_dir);
}

fn generate_output_filename(city: &str, theme_name: &str, theme_dir: &str) -> PathBuf {
    let now: OffsetDateTime = SystemTime::now().into();
    let timestamp = now.format(&Rfc3339).unwrap();
    let city_slug = city.to_lowercase().replace(" ", "_");
    let filename = format!("{}_{}_{}.png", city_slug, theme_name, timestamp);

    let mut out_path = PathBuf::new();
    out_path.push(theme_dir);
    out_path.push(filename);
    out_path
}
