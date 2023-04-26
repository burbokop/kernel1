use std::env;


fn get_os_from_triple(triple: &str) -> Option<&str> {
    triple.splitn(3, "-").nth(2)
}

fn main() {
    slint_build::compile_with_config(
        "src/slint/main.slint",
        slint_build::CompilerConfiguration::new()
            .with_style("fluent-dark".to_owned())
            .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer),
    )
    .unwrap();


    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    if let Some(target_os) = get_os_from_triple(&target) {
        if target_os.contains("windows") {
            println!("cargo:rustc-link-lib=static=SDL2main");
            //println!("cargo:rustc-link-lib=static=SDL2-static");
            println!("cargo:rustc-link-lib=static=SDL2");
            return;
        } else if !target_os.contains("none") {
            println!("cargo:rustc-link-lib=static=SDL2main");
            println!("cargo:rustc-link-lib=static=SDL2");
            return;
        }
    }
    println!("cargo:rustc-link-lib=__cstd");
}
