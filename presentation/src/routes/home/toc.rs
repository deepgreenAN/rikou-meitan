use dioxus::prelude::*;
use gloo_intersection::IntersectionObserverHandler;
use gloo_utils::document;
use indexmap::IndexMap;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::Element as WebSysElement;
use web_sys::{ScrollBehavior, ScrollIntoViewOptions};

#[derive(Props)]
pub struct TocContentProps<'a> {
    #[props(into)]
    pub title: String,
    #[props(into)]
    pub id: String,
    pub children: Element<'a>,
}

pub fn TocContent<'a>(cx: Scope<'a, TocContentProps<'a>>) -> Element {
    cx.render(rsx! {
        div { id:"{cx.props.id}" , class: "toc-content",
            h2 { class: "toc-content-title", "{cx.props.title}"},
            &cx.props.children
        }
    })
}

pub fn Toc(cx: Scope) -> Element {
    let toc_content_elements = use_state(cx, Vec::<WebSysElement>::new);
    let visible_map = cx.use_hook(|| Rc::new(RefCell::new(IndexMap::<String, bool>::new())));
    let active_id = use_state(cx, || Option::<String>::None);
    let intersection_observer_handler =
        cx.use_hook(|| Rc::new(Cell::new(Option::<IntersectionObserverHandler>::None)));

    use_effect(cx, (), {
        to_owned![
            toc_content_elements,
            visible_map,
            active_id,
            intersection_observer_handler
        ];

        |_| async move {
            let node_list = document().query_selector_all(".toc-content").unwrap_throw();
            let elements = (0..node_list.length())
                .map(|i| {
                    let element: WebSysElement = node_list.item(i).unwrap_throw().unchecked_into();
                    {
                        visible_map.borrow_mut().insert(element.id(), false);
                    }
                    element
                })
                .collect::<Vec<_>>();

            toc_content_elements.set(elements);

            let handler = IntersectionObserverHandler::new(move |entries, _| {
                entries.into_iter().for_each(|entry| {
                    if let Some(is_visible) = visible_map.borrow_mut().get_mut(&entry.target().id())
                    {
                        *is_visible = entry.is_intersecting();
                    }
                });
                {
                    let visible_map = visible_map.borrow();
                    let (found_key, _found_value) = visible_map
                        .iter()
                        .find(|(_id, is_visible)| **is_visible)
                        .unwrap_throw();
                    active_id.set(Some(found_key.clone()));
                }
            })
            .unwrap_throw();

            for i in 0..node_list.length() {
                handler.observe(&(node_list.item(i).unwrap_throw().unchecked_into()));
            }

            intersection_observer_handler.set(Some(handler));
        }
    });

    cx.render(rsx! {
        nav{
            id: "toc-container", aria_label:"Table of Contents",
            toc_content_elements.get().iter().map(|element|{
                let mut class = "toc-item".to_string();
                if let Some(active_id) = active_id.get() {
                    if element.id() == *active_id {
                        class.push_str(" active");
                    } else {
                        class.push_str(" not-active");
                    }
                }

                let title_str = element.query_selector("h2").unwrap_throw().unwrap_throw().inner_html();

                let mut scroll_options = ScrollIntoViewOptions::new();
                scroll_options.behavior(ScrollBehavior::Smooth);

                rsx! {
                    div { key: "{element.id()}", class:"{class}",
                        onclick: move |_|{element.scroll_into_view_with_scroll_into_view_options(&scroll_options)},
                        "{title_str}"
                    }
                }
            })
        }
    })
}
