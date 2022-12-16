use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        {"hello world!"}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
