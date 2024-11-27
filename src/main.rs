use racing_tournament::{settings, startup::Application, telemetry};
use std::io;
use tracing::{error, info, warn};

// use racing_tournament::

#[actix_web::main]
async fn main() -> io::Result<()> {
    // This is a macro that allows for multiple loggers to be used at once

    dotenv::dotenv().ok();

    let mut settings = match settings::get() {
        Ok(settings) => settings,
        Err(err) => {
            println!("Failed to load settings: {err}");
            panic!("Failed to load settings");
        }
    };

    let subscriber = telemetry::get_subcriber(settings.clone().debug);
    telemetry::init_subscriber(subscriber);

    info!("Building the application");
    let application = match Application::build(&mut settings).await {
        Ok(app) => app,
        Err(err) => {
            error!("Failed to build application: {err}");
            panic!("Failed to build application");
        }
    };

    info!("Listening on port: {}", application.port());
    application.run_until_stopped().await?;
    warn!("Shutting down");

    Ok(())
}

// Collect funds and apply discount for early payers

// Once the driver has paid, enter their name for random assignment

// Calculate the payout for all indvididuals working for the organization and the gross income of the event

// Calculate the initial odds; the reference is each drivers performance at this event only

// Ensure each driver's car passes the technical safety inspection

// Seed the tournament

// Loop
// Take bets from the spectators; Calculate the house fee

// Race!

// Calculate bet payouts
// End Loop

// Reseed the bracket

// BFCSP
// for $59 tuition for CS Prep? Until 12/2!
