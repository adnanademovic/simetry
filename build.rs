use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=src/raceroom_racing_experience/r3e.h");

    let bindings = bindgen::Builder::default()
        .header("src/raceroom_racing_experience/r3e.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .derive_partialeq(true)
        .derive_eq(true)
        .derive_hash(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("r3e.rs"))
        .expect("Couldn't write bindings!");

    let bindings = bindgen::Builder::default()
        .header("src/truck_simulator/scs-sdk-plugin/scssdk.h")
        .header("src/truck_simulator/scs-sdk-plugin/scs-telemetry-common.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .derive_partialeq(true)
        .derive_eq(true)
        .derive_hash(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("scs_telemetry_common.rs"))
        .expect("Couldn't write bindings!");
}
