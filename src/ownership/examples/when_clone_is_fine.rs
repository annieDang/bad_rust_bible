// Book I, Commandment I — Clone Thy Neighbour
// WHEN IT'S FINE: legitimate, deliberate uses of .clone().
//
// Run: cargo run --example when_clone_is_fine

use std::thread;

#[derive(Clone, Debug)]
struct AppConfig {
    host: String,
    port: u16,
}

fn main() {
    // ── 1. Thread boundaries ──────────────────────────────────────────────
    // thread::spawn requires 'static ownership. If you need the data in
    // both the spawned thread AND the current one, you must clone.
    let config = AppConfig {
        host: String::from("localhost"),
        port: 8080,
    };

    let thread_config = config.clone(); // deliberate: two real owners needed
    let handle = thread::spawn(move || {
        println!("Thread using: {}:{}", thread_config.host, thread_config.port);
    });

    println!("Main using:   {}:{}", config.host, config.port);
    handle.join().unwrap();

    // ── 2. Building independent copies you intend to mutate separately ────
    let base_headers: Vec<String> = vec![
        String::from("Content-Type: application/json"),
        String::from("Accept: application/json"),
    ];

    let mut auth_headers = base_headers.clone(); // divergence is intentional
    auth_headers.push(String::from("Authorization: Bearer <token>"));

    println!("\nBase headers:  {:?}", base_headers);
    println!("Auth headers:  {:?}", auth_headers);

    // ── 3. Cheap types where clone is trivial ─────────────────────────────
    // PathBuf, small Strings, enum variants — cloning these in non-hot paths
    // is fine; it keeps the code readable without meaningful overhead.
    let path = std::path::PathBuf::from("/etc/app/config.toml");
    let backup_path = path.clone(); // clear intent: two paths, two owners
    println!("\nConfig path: {:?}", path);
    println!("Backup path: {:?}", backup_path);

    // ── 4. Tests ──────────────────────────────────────────────────────────
    // (see tests module below) — in tests, readability beats nanoseconds.
}

#[cfg(test)]
mod tests {
    use super::AppConfig;

    fn default_config() -> AppConfig {
        AppConfig {
            host: String::from("localhost"),
            port: 8080,
        }
    }

    #[test]
    fn test_port_override() {
        let mut cfg = default_config();
        cfg.port = 9090;
        assert_eq!(cfg.port, 9090);
    }

    #[test]
    fn test_host_is_localhost() {
        let cfg = default_config();
        assert_eq!(cfg.host, "localhost");
    }
}
