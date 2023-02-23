use dioxus::prelude::*;
use dioxus_router::Link;

#[derive(Props, PartialEq)]
pub struct MoreButtonProps {
    #[props(into)]
    to: String,
}

pub fn MoreButton(cx: Scope<MoreButtonProps>) -> Element {
    cx.render(rsx! {
        Link {
            class: "more-button-container"
            to: "{cx.props.to}",
            "もっと見る"
        }
    })
}
