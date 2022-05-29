
#[macro_use]
extern crate macro_sandbox_lib;
// use log::{debug, info, trace, warn};
use log;
use std::error::Error;
use opentelemetry::sdk::trace::Tracer;
use opentelemetry::trace::TraceError;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;


//add this function
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

#[route(GET, "/")]
fn first_function() {
    println!("first_function")
}

#[show_streams]
fn second_function() {
    println!("second_function")
}
fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    println!("Hello, world!");
    let tracer = init_tracer().expect("Failed to initialize tracer"); //calling our new init_tracer function

    tracing_subscriber::registry() //(1)
        .with(tracing_subscriber::EnvFilter::new("TRACE")) //(2)
        .with(tracing_opentelemetry::layer().with_tracer(tracer)) //(3)
        .try_init()
        .expect("Failed to register tracer with registry");

    
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
    

    first_function();
    second_function();


    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    info!(number_of_yaks, "preparing to shave yaks");

    let number_shaved = shave_all(number_of_yaks);
    info!(
        all_yaks_shaved = number_shaved == number_of_yaks,
        "yak shaving completed."
    );

    opentelemetry::global::shutdown_tracer_provider(); //add this line
    Ok(())
}
//
// fn main() {
//     println!("Hello, world!");
//     first_function();
//     second_function();
// }
