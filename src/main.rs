use yew::prelude::*;

pub mod rust_conf;
pub mod counter;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <rust_conf::RustConf></rust_conf::RustConf>
            <counter::Counter></counter::Counter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}