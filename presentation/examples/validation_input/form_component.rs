use crate::domain_form::{DomainForm, DomainFormOpt};
use crate::validation_input_component::ValidationInputText;
use dioxus::prelude::*;
use log::info;
use std::cell::RefCell;
use std::rc::Rc;

pub fn ValidationForm(cx: Scope) -> Element {
    let domain_form_opt = Rc::new(RefCell::new(DomainFormOpt::default()));
    let first_submitted = use_state(cx, || false);

    let onclick = {
        to_owned![domain_form_opt];
        move |_| {
            if !first_submitted.get() {
                first_submitted.set(true);
            }

            let res: Result<DomainForm, _> = domain_form_opt
                .replace_with(|domain_form_opt| domain_form_opt.clone())
                .try_into();
            if let Ok(domain_form) = res {
                // 擬似敵な値の送信
                info!("form: {:?}", domain_form);
            }
        }
    };

    cx.render(rsx! {
        div {
            label {
                "郵便番号:"
                ValidationInputText {
                    onchange: {
                        to_owned![domain_form_opt];
                        move |opt|{domain_form_opt.borrow_mut().postal_code = opt;}
                    }
                    error_message: "有効な郵便番号ではありません".to_string()
                    required: true
                    show_error_message: *first_submitted.get()
                }
            }
        }
        div {
            label {
                "年月日:"
                ValidationInputText {
                    onchange: {
                        to_owned![domain_form_opt];
                        move |opt|{domain_form_opt.borrow_mut().date = opt;}
                    }
                    error_message: "有効な年月日ではありません".to_string()
                    required: true
                    show_error_message: *first_submitted.get()
                }
            }
        }
        div {button {onclick:onclick, "送信"}}
    })
}
