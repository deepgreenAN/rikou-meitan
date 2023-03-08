use dioxus::prelude::*;

#[derive(Props)]
pub struct EditModalProps<'a> {
    // キャプション
    caption: String,
    // // キャンセル時の挙動
    on_cancel: EventHandler<'a>,
    // 削除時の挙動
    on_remove: Option<EventHandler<'a>>,
    // 入力部分の要素
    input: Element<'a>,
    // プレビュー部分の要素
    preview: Element<'a>,
}

pub fn EditModal<'a>(cx: Scope<'a, EditModalProps<'a>>) -> Element {
    let is_preview_show = use_state(cx, || false);

    cx.render(rsx! {
        div { class: "edit-container",
            onclick: move |_|{cx.props.on_cancel.call(())}, //なぜかonmousedownのstop_propagationが効かない
            div {
                class: "edit-ui-container",
                onclick: move |e| {e.stop_propagation()},
                div { class: "edit-input-container",
                    div { class: "edit-input-caption", "{cx.props.caption}"}
                    &cx.props.input
                    div { class: "edit-input-bottom",
                        button { onclick:move |_|{is_preview_show.set(true)}, "プレビューを表示"}
                        button { onclick: move |_|{cx.props.on_cancel.call(())}, "キャンセル"}
                        // 削除ボタン
                        cx.props.on_remove.as_ref().map(|on_remove|{
                            rsx!{
                                button { onclick: move |_|{on_remove.call(())}, "削除"}
                            }
                        })
                    }
                }
                is_preview_show.get().then(||{
                    rsx!{
                        div { class:"edit-preview-container",
                            div { class: "edit-preview-caption", "プレビュー"}
                            &cx.props.preview
                        }
                    }
                })

            }
        }
    })
}
