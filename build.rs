
fn main() {
    slint_build::compile_with_config(
        "src/slint/main.slint",
        slint_build::CompilerConfiguration::new()
            .with_style("fluent-dark".to_owned())
            .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer),
    )
    .unwrap();

    println!("cargo:rustc-link-arg=-nodefaultlibs");
    println!("cargo:rustc-link-arg=-nolibc");


}
