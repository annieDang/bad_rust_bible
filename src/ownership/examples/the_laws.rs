// Book I, Commandment I — Clone Thy Neighbour
// THE LAWS: Rust's three ownership rules + borrowing rules, demonstrated.
//
// Run: cargo run --example the_laws

fn main() {
    // ── Law 1: every value has exactly one owner ──────────────────────────
    let s1 = String::from("hello");
    println!("Owner: s1 = \"{}\"", s1);

    // ── Law 2: ownership can be moved, but only one owner at a time ───────
    let s2 = s1; // s1 is MOVED into s2 — s1 is now invalid
    println!("Owner transferred: s2 = \"{}\"", s2);

    // Uncommenting the next line would be a compile error:
    // println!("{}", s1); // error[E0382]: borrow of moved value: `s1`

    // ── Law 3: when the owner goes out of scope, memory is freed ──────────
    {
        let scoped = String::from("I live briefly");
        println!("Inside scope: \"{}\"", scoped);
    } // `scoped` is dropped here — no GC, no leak

    // ── Clone: explicit deep copy (two separate heap allocations) ─────────
    let original = String::from("hello");
    let cloned = original.clone();
    println!("Both valid after clone: \"{}\" and \"{}\"", original, cloned);

    // ── Copy types: stack-only, always duplicated automatically ───────────
    let x: i32 = 42;
    let y = x; // COPY — x is still valid, nothing moved
    println!("Copy type: x={}, y={} (both valid)", x, y);

    // ── Borrowing rule 1: many immutable borrows at once ──────────────────
    let s = String::from("shared");
    let r1 = &s;
    let r2 = &s;
    println!("Two immutable borrows: \"{}\" and \"{}\"", r1, r2);

    // ── Borrowing rule 2: exactly one mutable borrow, no others ──────────
    let mut m = String::from("mutable");
    {
        let r_mut = &mut m;
        r_mut.push_str(" world");
        println!("After mutable borrow: \"{}\"", r_mut);
    } // r_mut goes out of scope — m is ours again
    println!("Back in full control: \"{}\"", m);

    // Uncommenting this would be a compile error (mixed refs):
    // let r_immut = &m;
    // let r_mut2  = &mut m; // error[E0502]: cannot borrow `m` as mutable
    // println!("{} {}", r_immut, r_mut2);
}
