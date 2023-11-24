fn main() {
    // https://docs.rs/sqlx/latest/sqlx/macro.migrate.html#stable-rust-cargo-build-script
    println!("cargo:rerun-if-changed=migrations");
    tauri_build::build()
}
