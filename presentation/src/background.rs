use dioxus::prelude::*;
use fermi::use_read;

#[derive(Props)]
pub struct BackgroundProps<'a> {
    pub children: Element<'a>,
}

pub fn Background<'a>(cx: Scope<'a, BackgroundProps<'a>>) -> Element {
    let is_dark_mode = use_read(cx, crate::IS_DARK_MODE);

    let (background_grad_class, background_pattern_class) = match *is_dark_mode {
        true => ("background-grad-dark", "background-pattern-dark"),
        false => ("background-grad-light", "background-pattern-light"),
    };

    cx.render(rsx! {
        div { id: "background-container",
            div { id: "background-grad", class: "{background_grad_class}"},
            div { id: "background-svg",
                // svg {
                //     view_box: "0 0 100 100",
                //     defs {
                //         circle {cx: "50", cy: "50", r: "30", style: "fill:black;"}
                //     }
                // }
            },
            div { id: "background-pattern", class: "{background_pattern_class}"},
            &cx.props.children
        }
    })
}
