use crate::components::AddButton;

use dioxus::prelude::*;

#[derive(Props)]
pub struct VideoPageMenuProps<'a, E>
where
    E: strum::IntoEnumIterator + ToString,
{
    // AddButtonを押されたときの処理
    on_click_add_button: EventHandler<'a>,
    // ソートのセレクトボックスが変更したときの処理
    on_change_sort_select: EventHandler<'a, FormEvent>,
    // dioxusには型パラメーターの直接指定が困難なため利用
    _enum_type: E,
}

pub fn VideoPageMenu<'a, E>(cx: Scope<'a, VideoPageMenuProps<'a, E>>) -> Element
where
    E: strum::IntoEnumIterator + ToString,
{
    cx.render(rsx! {
        div { class: "video-page-menu",
            div { class: "sort-select-container",
                select { onchange: move |e|{cx.props.on_change_sort_select.call(e)},
                    E::iter().map(|variant|{
                        let variant_str = variant.to_string();
                        rsx!{
                            option { class: "sort-option", value: "{variant_str}", "{variant_str}"}
                        }
                    })
                }
            }
            div { class: "add-button",
                AddButton{onclick: move |_|{cx.props.on_click_add_button.call(())}}
            }
        }
    })
}
