use std::path::Path;


async fn download(url: &str, dst: &str) {
    use std::{io, fs::File};
    let resp = reqwest::get(url).await.expect("request failed");
    let body = resp.text().await.expect("body invalid");
    std::fs::create_dir_all(Path::new(dst).parent().expect("dst dir has no parent")).expect("can not create dirs");
    let mut out = File::create(dst).expect("failed to create file");
    io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
}

#[tokio::main]
async fn main() {
    download("https://upload.wikimedia.org/wikipedia/commons/f/f7/Bananas.svg", "assets/img/bananas.svg").await;

    slint_build::compile_with_config(
        "src/slint/main.slint",
        slint_build::CompilerConfiguration::new()
            .with_style("fluent-light".to_owned())
            .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer),
    )
    .unwrap();

    println!("cargo:rustc-link-arg=-nodefaultlibs");
    println!("cargo:rustc-link-arg=-nolibc");


}
