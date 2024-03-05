use yew::prelude::*;



#[function_component]
pub fn Calculator() -> Html {
    let state = use_state(|| 0);
    
    html! {
        <>
            <h1>{"Calculator"}</h1>
            <button onclick={Callback::from({
                let state = state.clone();
                move |_| {
                    let val = *state.clone();
                    state.set(utils::add_val(val, 1));
                }
            })}>{"+1"}</button>
            <button onclick={Callback::from({
                let state = state.clone();
                move |_| {
                    let val = *state.clone();
                    state.set(utils::add_val(val, 2));
                }
            })}>{"+2"}</button>
            <button onclick={Callback::from({
                let state = state.clone();
                move |_| {
                    let val = *state.clone();
                    state.set(utils::add_val(val, 3));
                }
            })}>{"+3"}</button>
            <br />
            <button onclick={Callback::from({
                let state = state.clone();
                move |_| {
                    let val = *state.clone();
                    state.set(utils::add_val(val, 4));
                }
            })}>{"+4"}</button>
            <button onclick={Callback::from({
                let state = state.clone();
                move |_| {
                    let val = *state.clone();
                    state.set(utils::add_val(val, 5));
                }
            })}>{"+5"}</button>
            <button onclick={Callback::from({
                let state = state.clone();
                move |_| {
                    let val = *state.clone();
                    state.set(utils::add_val(val, 6));
                }
            })}>{"+6"}</button>
            <br />
            <button onclick={Callback::from({
                let state = state.clone();
                move |_| {
                    let val = *state.clone();
                    state.set(utils::add_val(val, 7));
                }
            })}>{"+7"}</button>
            <button onclick={Callback::from({
                let state = state.clone();
                move |_| {
                    let val = *state.clone();
                    state.set(utils::add_val(val, 8));
                }
            })}>{"+8"}</button>
            <button onclick={Callback::from({
                let state = state.clone();
                move |_| {
                    let val = *state.clone();
                    state.set(utils::add_val(val, 9));
                }
            })}>{"+9"}</button>
            <br />
            <button onclick={Callback::from({
                let state = state.clone();
                move |_| {
                    let val = *state.clone();
                    state.set(utils::add_val(val, 0));
                }
            })}>{"+0"}</button>
            <p>{"I know, not a calculator, but a work in progress."}</p>
            <h1>{format!("Result: {}", *state.clone())}</h1>
        </>
    }
}

pub mod utils {
    pub fn add_val(val :i32, num :i32) -> i32 {
        val + num
    }
}

// #[derive(Properties, PartialEq)]
// struct CalcButtonProps {
//     result :UseStateHandle<i32>,
//     value :i32,
// }

// #[function_component]
// fn CalcButton(CalcButtonProps { result, value } :&CalcButtonProps) -> Html {
//     let on_button_click = Callback::from(move |_| {
        
//     });

//     html! {
//         <button onclick={on_button_click}></button>
//     }
// }

// #[derive(Properties, PartialEq)]
// struct CalcProps {
//     on_click: Callback<i32>,
// }

// #[function_component]
// pub fn Calculator() -> Html {
//     let result = use_state(|| 0);
//     let display_result = *result.clone();

//     let on_button_click = Callback::from(move |e: MouseEvent| {
//         let target = e.target().unwrap();
//         let id = target.
//         let value = *result + 1;
//         result.set(value);
//     });

//     html! {
//         <>
//             <h1>{ "Calculator" }</h1>
//             <button id={"1"} onclick={on_button_click.clone()}>{"1"}</button>
//             <button id={"1"} onclick={on_button_click.clone()}>{"2"}</button>
//             <button id={"1"} onclick={on_button_click.clone()}>{"3"}</button>
//             <br />
//             <button id={"1"} onclick={on_button_click.clone()}>{"4"}</button>
//             <button id={"1"} onclick={on_button_click.clone()}>{"5"}</button>
//             <button id={"1"} onclick={on_button_click.clone()}>{"6"}</button>
//             <br />
//             <button id={"1"} onclick={on_button_click.clone()}>{"7"}</button>
//             <button id={"1"} onclick={on_button_click.clone()}>{"8"}</button>
//             <button id={"1"} onclick={on_button_click.clone()}>{"9"}</button>
//             <br />
//             <h2>{ format!("Result: {}", display_result) }</h2>
//         </>
//     }
// }
