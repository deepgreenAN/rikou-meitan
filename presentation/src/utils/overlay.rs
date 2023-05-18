use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;
use gloo_events::EventListener;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Event, HtmlElement};

const OVERLAY_PARENT_ID: &str = "background-container";

/// オーバーレイを扱う状態
#[derive(Clone, Debug)]
pub struct OverlayElement {
    element: Rc<RefCell<Option<HtmlElement>>>,
    z_index: i32,
    event_listeners: Rc<RefCell<Vec<EventListener>>>,
}

impl OverlayElement {
    pub fn new(z_index: i32) -> Self {
        Self {
            element: Rc::new(RefCell::new(None)),
            z_index,
            event_listeners: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// オーバーレイをアクティブ化する
    pub fn activate(&self) -> Result<(), JsValue> {
        let mut element_mut = self.element.borrow_mut();

        if element_mut.is_none() {
            let new_element = gloo_utils::document()
                .create_element("div")?
                .unchecked_into::<HtmlElement>();
            new_element.set_class_name("overlay");
            let new_element_style = new_element.style();
            new_element_style.set_property("z-index", &format!("{}", self.z_index))?; // z-indexを設定
            let parent = gloo_utils::document()
                .get_element_by_id(OVERLAY_PARENT_ID)
                .ok_or(JsValue::from("Cannot get #background-container"))?;

            parent.append_child(new_element.as_ref())?;

            *element_mut = Some(new_element);
        }

        Ok(())
    }
    /// オーバーレイにイベントリスナーを追加．アクティブ化してから利用しないと意味がない
    pub fn add_event_listener<S: Into<Cow<'static, str>>, F: FnMut(&Event) + 'static>(
        &self,
        event_type: S,
        callback: F,
    ) {
        if let Some(element) = self.element.borrow().as_ref() {
            let event_listener = EventListener::new(element, event_type, callback);
            self.event_listeners.borrow_mut().push(event_listener);
        }
    }
    /// オーバーレイを非アクティブ化する
    pub fn deactivate(&self) {
        let element = self.element.borrow_mut().take();
        if let Some(element) = element {
            element.remove();
        }

        self.event_listeners.borrow_mut().clear();
    }
}

impl Drop for OverlayElement {
    fn drop(&mut self) {
        if let Some(element) = self.element.borrow().as_ref() {
            element.remove(); // DOMツリーから削除
        }
    }
}

pub fn use_overlay<T>(cx: Scope<'_, T>, z_index: i32) -> &'_ OverlayElement {
    cx.use_hook(|| OverlayElement::new(z_index))
}
