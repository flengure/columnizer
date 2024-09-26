use std::env;

fn main() {
    // Fetch the LC_NUMERIC locale setting
    if let Ok(locale_numeric) = env::var("LC_NUMERIC") {
        println!("LC_NUMERIC: {}", locale_numeric);
    } else {
        println!("LC_NUMERIC not set. Falling back to LANG.");

        // If LC_NUMERIC is not set, try LANG
        if let Ok(locale) = env::var("LANG") {
            println!("LANG: {}", locale);
        } else {
            println!("No locale information found.");
        }
    }
}
