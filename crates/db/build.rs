use std::env;
use std::path::Path;

fn main() {
    cornucopia();
}

fn cornucopia() {
    let queries_path = "queries";

    let out_dir = env::var("OUT_DIR").unwrap();
    let file_path = Path::new(&out_dir).join("cornucopia.rs");

    let db_url = env::var_os("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("cargo:rerun-if-changed={}", queries_path);

    let output = std::process::Command::new("cornucopia")
        .arg("--queries-path")
        .arg(queries_path)
        .arg("--serialize")
        .arg("-d")
        .arg(&file_path)
        .arg("live")
        .arg(db_url)
        .output()
        .unwrap();

    if !output.status.success() {
        panic!("{}", &std::str::from_utf8(&output.stderr).unwrap());
    }
}
