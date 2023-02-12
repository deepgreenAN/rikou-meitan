use dioxus::prelude::*;

pub fn TitleLogo(cx: Scope) -> Element {
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
        div { id: "title-logo",
            div { id: "bottle-svg",
                dangerous_inner_html: "{bottle_svg}"
            }
            div { id: "double-heart-svg",
                dangerous_inner_html: "{heart}"
            }
            div { id: "moon-svg",
                dangerous_inner_html: "{moon_svg}"
            }
            div { id: "logo-text-svg",
                dangerous_inner_html: "{logo_text}"
            }
        }
    })
}
