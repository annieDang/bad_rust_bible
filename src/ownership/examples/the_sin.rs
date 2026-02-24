// Book I, Commandment I — Clone Thy Neighbour
// THE SIN: cloning when a reference would do.
//
// Run: cargo run --example the_sin

#[derive(Clone)]
struct Config {
    database_url: String,
    max_connections: u32,
}

// ❌ THE SIN: takes ownership of the whole struct just to print one field.
//    Every caller is forced to .clone() before passing, or give up their value.
fn log_config_bad(config: Config) {
    println!("[BAD]  database_url = {}", config.database_url);
    // `config` is dropped here — the caller lost their value for nothing.
}

// ❌ ALSO BAD: cloning a String inside a function that already owns it.
fn connect_bad(url: String) {
    println!("[BAD]  connecting to {} ...", url.clone()); // clone is pointless here
}

// ✅ THE FIX: borrow the struct — we only need to read it.
fn log_config_good(config: &Config) {
    println!("[GOOD] database_url = {}", config.database_url);
}

// ✅ ALSO GOOD: accept &str when you only need to read a string.
fn connect_good(url: &str) {
    println!("[GOOD] connecting to {} ...", url);
}

fn main() {
    let config = Config {
        database_url: String::from("postgres://localhost/mydb"),
        max_connections: 5,
    };

    // --- bad path ---
    log_config_bad(config.clone()); // forced clone just to keep `config` alive
    connect_bad(config.database_url.clone()); // another forced clone

    // --- good path ---
    log_config_good(&config);          // borrow — config lives on
    connect_good(&config.database_url); // coerces &String → &str, zero cost
    log_config_good(&config);          // can still use config after
}
