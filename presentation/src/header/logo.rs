use dioxus::prelude::*;
use dioxus_router::Link;
use rand::{thread_rng, Rng};

pub fn TitleLogo(cx: Scope) -> Element {
    let is_active = 0.1 > thread_rng().gen::<f64>();

    let title_logo_class = if is_active { "active" } else { "inactive" };

    let bottle_svg = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/images/release/bottle-blackandpink-edge.svg"
    ));
    let moon_svg = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/images/release/moon-blackandyellow-edge.svg"
    ));
    let heart = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/images/release/double-heart.svg"
    ));
    let logo_text = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/images/release/logo-text.svg"
    ));

    cx.render(rsx! {
        div { id: "title-logo", class: "{title_logo_class}",
            div { id: "bottle-svg",
                dangerous_inner_html: "{bottle_svg}"
            }
            div { id: "double-heart-svg",
                dangerous_inner_html: "{heart}"
            }
            div { id: "moon-svg",
                dangerous_inner_html: "{moon_svg}"
            }
            Link { id: "logo-text-svg", to:"/",
                div {dangerous_inner_html: "{logo_text}"}
            }
        }
    })
}
