

use log;
use std::error::Error;
use opentelemetry::sdk::trace::Tracer;
use opentelemetry::trace::TraceError;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;


//Init the Tracer
fn init_tracer() -> Result<Tracer, TraceError> {
    opentelemetry_jaeger::new_pipeline()
        .with_service_name("telem")
        .install_simple()
}

#[tracing::instrument]
fn shave_it(index: i32) {
    info!(current_yak=index, "Shaving...");
}

#[tracing::instrument]
pub(crate) fn shave_all(number_of_yaks: i32) -> i32 {
    for yak_index in 0..number_of_yaks {
        info!(current_yak=yak_index+1, "Shaving in progress");
        shave_it(yak_index+1);
    }

    number_of_yaks
}

fn first_function() {
    println!("first_function")
}

fn second_function() {
    println!("second_function")
}

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    println!("Hello, world!");

    // Start Tracer
    let tracer = init_tracer().expect("Failed to initialize tracer"); 
    tracing_subscriber::registry() 
        .with(tracing_subscriber::EnvFilter::new("TRACE")) 
        .with(tracing_opentelemetry::layer().with_tracer(tracer)) 
        .try_init()
        .expect("Failed to register tracer with registry");

    // Start Fern
    fern::Dispatch::new()
    // Perform allocation-free log formatting
    .format(|out, message, record| {
        out.finish(format_args!(
            "{}[{}][{}] {}",
            chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            record.target(),
            record.level(),
            message
        ))
    })
    // Add blanket level filter -
    .level(log::LevelFilter::Debug)
    // - and per-module overrides
    .level_for("hyper", log::LevelFilter::Info)
    // Output to stdout, files, and other Dispatch configurations
    .chain(std::io::stdout())
    .chain(fern::log_file("output.log")?)
    // Apply globally
    .apply()?;
    
    // Run some Functions
    first_function();
    second_function();

    // Tracer Tutorial
    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    info!(number_of_yaks, "preparing to shave yaks");

    let number_shaved = shave_all(number_of_yaks);
    info!(
        all_yaks_shaved = number_shaved == number_of_yaks,
        "yak shaving completed."
    );

    opentelemetry::global::shutdown_tracer_provider(); 
    Ok(())
}

