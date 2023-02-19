use dioxus::prelude::*;
use gloo_events::{EventListener, EventListenerOptions};
use std::cell::Cell;
use std::rc::Rc;

// スクロールのロックを行う状態
#[derive(Clone, Default)]
pub struct ScrollLockState {
    event_listeners: Rc<Cell<Option<Vec<EventListener>>>>,
}

impl ScrollLockState {
    pub fn new() -> Self {
        Self {
            event_listeners: Rc::new(Cell::new(None)),
        }
    }
    // スクロールをロックする．
    pub fn lock(&self) {
        let document = gloo_utils::document();
        let options = EventListenerOptions {
            passive: false,
            ..Default::default()
        };

        let event_listeners = vec![
            EventListener::new_with_options(&document, "wheel", options, move |e| {
                e.prevent_default();
            }),
            EventListener::new_with_options(&document, "touchmove", options, move |e| {
                e.prevent_default();
            }),
        ];

        self.event_listeners.set(Some(event_listeners));
    }
    // スクロールをアンロックする．
    pub fn unlock(&self) {
        self.event_listeners.take();
    }
}

pub fn use_scroll_lock(cx: Scope<'_>) -> &'_ ScrollLockState {
    cx.use_hook(|_| ScrollLockState::new())
}
