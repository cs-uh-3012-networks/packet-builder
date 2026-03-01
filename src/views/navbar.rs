use crate::theme::Theme;
use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    let mut theme = use_context::<Signal<Theme>>();

    let icon = match *theme.read() {
        Theme::Dark => "\u{2600}\u{fe0f}",  // sun
        Theme::Light => "\u{1f319}", // moon
    };

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Packet Builder"
            }
            button {
                class: "theme-toggle",
                title: "Toggle light/dark mode",
                onclick: move |_| theme.write().toggle(),
                "{icon}"
            }
        }

        Outlet::<Route> {}
    }
}
