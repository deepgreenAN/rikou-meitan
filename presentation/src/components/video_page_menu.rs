use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use crate::components::AddButton;

use dioxus::prelude::*;

#[derive(Props)]
pub struct VideoPageMenuProps<'a, T>
where
    T: strum::IntoEnumIterator + ToString + FromStr,
{
    // AddButtonを押されたときの処理
    on_click_add_button: EventHandler<'a>,
    // ソートのセレクトボックスが変更したときの処理
    on_change_sort_select: EventHandler<'a, T>,
}

pub fn VideoPageMenu<'a, T, E>(cx: Scope<'a, VideoPageMenuProps<'a, T>>) -> Element
where
    T: strum::IntoEnumIterator + ToString + FromStr<Err = E>,
    E: Display + Debug,
{
    cx.render(rsx! {
        div { class: "video-page-menu",
            div { class: "sort-select-container",
                select {
                    onchange: move |e|{
                        let sort_type: T = e.value.parse().expect("Enum Parse Error");
                        cx.props.on_change_sort_select.call(sort_type);
                    },
                    T::iter().map(|variant|{
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
