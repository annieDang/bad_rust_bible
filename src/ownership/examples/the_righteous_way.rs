// Book I, Commandment I — Clone Thy Neighbour
// THE RIGHTEOUS WAY: prefer references; use Arc when you genuinely need shared ownership.
//
// Run: cargo run --example the_righteous_way

use std::sync::Arc;
use std::thread;

#[derive(Debug)]
struct Config {
    database_url: String,
    max_connections: u32,
}

// ✅ Borrow: caller keeps ownership, no allocation.
fn connect(database_url: &str) {
    println!("Connecting to: {}", database_url);
}

// ✅ Borrow the whole struct: read multiple fields without consuming it.
fn log_config(config: &Config) {
    println!(
        "Config {{ url: {}, max_conn: {} }}",
        config.database_url, config.max_connections
    );
}

// ✅ Prefer &[T] over &Vec<T> — accepts both Vec and plain slices.
fn print_migrations(migrations: &[&str]) {
    for m in migrations {
        println!("  applying: {}", m);
    }
}

// ✅ Prefer &str over &String — accepts both String and str literals.
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let raw_url = String::from("postgres://localhost/mydb");
    let config = Config {
        database_url: raw_url, // moved in — no clone
        max_connections: 5,
    };

    connect(&config.database_url); // borrow a field
    log_config(&config);           // borrow the struct
    log_config(&config);           // can borrow again — we still own it

    let migrations = vec!["001_create_users", "002_add_email_index"];
    print_migrations(&migrations); // &Vec<T> coerces to &[T] automatically

    greet("Ferris");                           // string literal (&str)
    greet(&String::from("Rustacean"));         // &String coerces to &str

    // ── Arc: share ownership across threads without cloning the data ──────
    // When you genuinely need multiple owners (e.g. threads), clone the
    // *pointer*, not the data. Arc::clone is a cheap atomic increment.
    let shared_config = Arc::new(Config {
        database_url: String::from("postgres://localhost/shared"),
        max_connections: 10,
    });

    let handles: Vec<_> = (0..3)
        .map(|i| {
            let cfg = Arc::clone(&shared_config); // cheap: increments ref-count
            thread::spawn(move || {
                println!(
                    "Thread {} sees url: {}",
                    i, cfg.database_url
                );
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
