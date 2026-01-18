use tracing::Level;

fn main() {
    let ansi_enabled = fix_ansi_term();

    tracing_subscriber::fmt()
        .with_ansi(ansi_enabled)
        .with_max_level(Level::DEBUG)
        .init();
}

#[cfg(target_os = "windows")]
fn fix_ansi_term() -> bool {
    nu_ansi_term::enable_ansi_support().is_ok_and(|()| true)
}

#[cfg(not(target_os = "windows"))]
fn fix_ansi_term() -> bool {
    true
}
