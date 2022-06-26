fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=res/wix_rules_template.wxs");
    std::fs::write(
        "wix_rules.wxs",
        format!(
            include_str!("res/wix_rules_template.wxs"),
            app_version = env!("CARGO_PKG_VERSION"),
            app_name = env!("CARGO_PKG_NAME"),
            app_manufacturer = env!("CARGO_PKG_AUTHORS"),
        ),
    )
    .unwrap();
}
