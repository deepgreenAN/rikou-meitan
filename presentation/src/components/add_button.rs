use dioxus::prelude::*;

#[derive(Props)]
pub struct AddButtonProps<'a> {
    onclick: Option<EventHandler<'a>>,
}

pub fn AddButton<'a>(cx: Scope<'a, AddButtonProps<'a>>) -> Element {
    let add_button_svg_str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/images/release/add_button.svg"
    ));

    cx.render(rsx! {
        div {class: "add-button-container",
            onclick: move |_|{
                if let Some(onclick) = &cx.props.onclick {
                    onclick.call(());
                }
            },
            div { class:"add-button-svg",
                dangerous_inner_html: "{add_button_svg_str}"
            }
            div { class:"add-button-text",
                "新規追加"
            }
        }
    })
}
