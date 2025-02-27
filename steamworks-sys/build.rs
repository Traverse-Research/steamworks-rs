//! Generates bindings for steamworks sdk (1.61)
//!
//! The steamworks dynamic library is copied to the target directory upon invoking this build script. A
//! user is expected to load the dynamic library at runtime using `libloading` crate.

#[cfg(feature = "rebuild-bindings")]
extern crate bindgen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    // https://github.com/rust-lang/cargo/issues/9661
    // Typically points to: $PROJECT/$TARGET_DIR/(x86_64-pc-windows-msvc/)$PROFILE/build/project-495fd7493ea5486b/out
    // CARGO_(BUILD_)TARGET_DIR is only set by the `cargo` user, `cargo` itself **DOES NOT** set it
    // when the user passes --target-dir: https://doc.rust-lang.org/cargo/reference/config.html#buildtarget-dir
    let target_dir = std::path::Path::new(&out_dir).join("../../..");

    let sdk_src = std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("lib")
        .join("steam");

    let triple = std::env::var("TARGET").unwrap();

    dbg!("TARGET TRIPLE: {triple}");

    let dylib_src = sdk_src.join("redistributable_bin").join({
        if triple.contains("windows") {
            if !triple.contains("i686") {
                // This dll has been renamed from `steam_api64` to `steam_api`.
                "win64/steam_api.dll"
            } else {
                panic!("Unsupported OS");
            }
        } else if triple.contains("linux") {
            if triple.contains("i686") {
                "linux32/libsteam_api.so"
            } else {
                "linux64/libsteam_api.so"
            }
        } else if triple.contains("darwin") {
            "osx/libsteam_api.dylib"
        } else {
            panic!("Unsupported OS");
        }
    });

    std::fs::copy(&dylib_src, target_dir.join(dylib_src.file_name().unwrap())).unwrap();

    #[cfg(feature = "rebuild-bindings")]
    {
        let target_os = if triple.contains("windows") {
            "windows"
        } else if triple.contains("darwin") {
            "macos"
        } else if triple.contains("linux") {
            "linux"
        } else {
            panic!("Unsupported OS");
        };
        let binding_path =
            std::path::Path::new(&format!("src/{}_bindings.rs", target_os)).to_owned();
        let bindings = bindgen::Builder::default()
            .header(
                sdk_src
                    .join("public/steam/steam_api_flat.h")
                    .to_string_lossy(),
            )
            .header(
                sdk_src
                    .join("public/steam/steam_gameserver.h")
                    .to_string_lossy(),
            )
            .allowlist_item("SteamAPI\\w+")
            .dynamic_library_name("steam_api")
            .clang_arg("-xc++")
            .clang_arg("-std=c++11")
            .clang_arg(format!("-I{}", sdk_src.join("public").display()))
            .default_enum_style(bindgen::EnumVariation::Rust {
                non_exhaustive: true,
            })
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file(binding_path)
            .expect("Couldn't write bindings!");
    }

    Ok(())
}
