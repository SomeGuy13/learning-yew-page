use yew::prelude::*;

#[function_component]
pub fn Calculator() -> Html {
    let state = use_state(|| String::new());
    let button_values :Vec<char> = vec!['1', '2', '3', '/', '4', '5', '6', '*', '1', '2', '3', '-', '0', '.', '=', 'C'];
    let buttons = button_values.iter().map(|value :&char| html! {
        <>
            <button onclick={Callback::from({
                let str1 = state.clone();
                let str2 = value.clone();
                let state = state.clone();
                move |_| {
                    state.set(format!("{}{}", *str1, str2));
                }
            })}>{format!("{}", value)}</button>
        </>
    });
    
    html! {
        <>
            <h1>{"Calculator"}</h1>
            <div>
                { for buttons }
            </div>
            <p>{"I know, not a calculator, but a work in progress."}</p>
            <h1>{format!("Result: {}", *state.clone())}</h1>
        </>
    }
}

pub mod utils {
    pub fn add_val(val :&i32, num :&i32) -> i32 {
        val + num
    }
}
