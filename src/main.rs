use std::process;

use dotenv;

use monair::MonairConfig;

fn main() {
    dotenv::dotenv().expect("Failed to read .env");

    match envy::from_env::<MonairConfig>() {
        Ok(config) => {
            if let Err(e) = monair::run(config) {
                eprint!("Application error: {}", e);
                process::exit(1);
            }

            ()
        }
        Err(e) => println!("Couldn't read config ({})", e),
    };
}
