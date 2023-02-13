use dioxus::prelude::*;

#[derive(Props)]
pub struct TocContentProps<'a> {
    pub title: String,
    pub children: Element<'a>,
}

pub fn TocContent<'a>(cx: Scope<'a, TocContentProps<'a>>) -> Element {
    cx.render(rsx! {
        div { class: "toc-content",
            h2 { class: "toc-content-title", "{cx.props.title}"},
            &cx.props.children
        }
    })
}

pub fn ToC(cx: Scope) -> Element {
    cx.render(rsx! {})
}
