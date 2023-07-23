// maud example
use maud::{html, Markup};
async fn hello_world2(
    templates: templates::Templates,
    config: State<config::ConfigContent>,
) -> Markup {
    // templates.home(&config);
    html! {
        head {
            style { r#"html, body { font-family: system-ui, sans-serif; } body { margin: 2rem auto; max-width: 500px; }"# }
        }
        body {
            h1 { "Welcome to your new installation of the Here Now server" }
            p { "You're now looking at the self-configuration page, where we'll set up your service."}
        }
    }
}
