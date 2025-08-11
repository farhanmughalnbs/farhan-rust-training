use log::{debug, info, warn, error};
fn main() {
    debug!("debug only when RUST_LOG=debug");
    info!("starting logging demo");
    warn!("this is a warning");
    error!("this is an error");
}
