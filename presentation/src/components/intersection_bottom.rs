use dioxus::prelude::*;
use gloo_intersection::IntersectionObserverHandler;
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;

#[allow(dead_code)]
#[derive(Props)]
pub struct IntersectionBottomProps<'a> {
    intersection_handler: Rc<IntersectionObserverHandler>,
    children: Element<'a>,
}

pub fn IntersectionBottom<'a>(cx: Scope<'a, IntersectionBottomProps<'a>>) -> Element {
    use_effect(cx, (), {
        let intersection_handler = cx.props.intersection_handler.clone();
        |_| async move {
            let target_element = gloo_utils::document()
                .query_selector("#intersection-bottom")
                .unwrap_throw()
                .unwrap_throw();
            intersection_handler.observe(&target_element);
        }
    });

    cx.render(rsx! {
        div {id: "intersection-bottom"}
    })
}
