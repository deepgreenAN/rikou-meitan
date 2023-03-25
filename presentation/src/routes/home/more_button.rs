use dioxus::prelude::*;
use dioxus_router::Link;
use web_sys::ScrollToOptions;

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
            onclick: move |_|{
                let mut option = ScrollToOptions::new();
                option.top(0.0);
                gloo_utils::window().scroll_to_with_scroll_to_options(&option);
            },
            "もっと見る"
        }
    })
}
