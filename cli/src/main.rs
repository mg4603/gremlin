use gremlin_core::logging;

fn main() {
    logging::init();

    tracing::info!("scanner starting");
}
