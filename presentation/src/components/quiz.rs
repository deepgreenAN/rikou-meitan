use dioxus::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Deserialize, Clone)]
pub struct QuizJson {
    question: String,
    answers: Vec<String>,
}

#[derive(Props)]
pub struct QuizProps<'a> {
    on_cancel: EventHandler<'a>,
    children: Element<'a>,
}

pub fn Quiz<'a>(cx: Scope<'a, QuizProps<'a>>) -> Element {
    let is_correct = use_state(cx, || false);
    let value_state = cx.use_hook(|| Rc::new(Cell::new(String::new())));

    let quiz_list_str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/contents/orikou_quiz.json"
    ));

    let quiz_list: Vec<QuizJson> =
        serde_json::from_str(quiz_list_str).expect("Quiz Deserialize sanity check");

    let quiz = quiz_list
        .choose(&mut thread_rng())
        .expect("Zero length sanity check")
        .clone();

    let onsubmit = {
        to_owned![value_state];
        move |_| {
            let value = value_state.take();
            if quiz.answers.iter().any(|answer| *answer == value) {
                is_correct.set(true);
            }
        }
    };

    cx.render(rsx! {
        if *is_correct.get() {
            rsx! {&cx.props.children}
        } else {
            rsx! {
                div { class: "quiz-container", onclick: move |_| {cx.props.on_cancel.call(())},
                    div { class: "quiz-ui-container", onclick: move |e|{e.stop_propagation();},
                        div { class: "quiz-caption", "おりコウクイズ"}
                        div { class: "quiz-desc", "※編集するにはクイズに正解してください(荒らし等を防ぐためです。分からない場合は管理人にお気軽にお尋ねください。)"}
                        div { class: "question", "{quiz.question}"}
                        div { class: "quiz-input-container",
                            div {"回答"}
                            input { class: "quiz-input", r#type:"text", onchange: move |e|{value_state.set(e.value.clone())}}
                        }
                        div { class: "quiz-bottom",
                            button {onclick: onsubmit, "回答する"}
                            button {onclick: move |_| {cx.props.on_cancel.call(())}, "キャンセル"}
                        }
                    }
                }
            }
        }
    })
}
