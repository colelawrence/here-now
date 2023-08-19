fn main() {
    let style = "fluent";
    slint_build::compile_with_config(
        "../ui/main_window.slint",
        slint_build::CompilerConfiguration::new()
            .with_style(style.to_string())
            .into(),
    )
    .unwrap();
}
