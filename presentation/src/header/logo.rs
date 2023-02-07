use dioxus::prelude::*;
use fermi::use_atom_state;

pub fn TitleLogo(cx: Scope) -> Element {
    let is_dark_mode = use_atom_state(&cx, crate::IS_DARK_MODE);
    // ロゴのカラー
    let logo_color = match is_dark_mode.get() {
        true => "#faeba7",
        false => "#3d003c",
    };

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
        "/images/release/title-logo.svg"
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
            div { id: "logo", color: "{logo_color}",
                dangerous_inner_html: "{logo_text}"
            }
        }
    })
}
