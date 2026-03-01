use dioxus::prelude::*;

use theme::Theme;
use views::{Home, Navbar};

mod components;
pub mod packet;
pub mod theme;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const THEME_CSS: Asset = asset!("/assets/styling/theme.css");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            dioxus::desktop::Config::default().with_window(
                dioxus::desktop::WindowBuilder::new()
                    .with_title("Packet Builder")
                    .with_inner_size(dioxus::desktop::LogicalSize::new(1024.0, 768.0))
                    .with_min_inner_size(dioxus::desktop::LogicalSize::new(1000.0, 500.0))
                    .with_always_on_top(false),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    let theme = use_signal(|| Theme::Dark);
    use_context_provider(|| theme);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: THEME_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: theme.read().as_str(),
            Router::<Route> {}
        }
    }
}
