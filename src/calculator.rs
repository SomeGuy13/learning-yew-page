use yew::prelude::*;

#[function_component]
pub fn Calculator() -> Html {
    let state = use_state(|| 0);
    let button_values :Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let buttons = button_values.iter().map(|value :&i32| html! {
        <>
            <button onclick={Callback::from({
                let state = state.clone();
                let b_value = value.clone();
                move |_| {
                    let val = *state.clone();
                    state.set(utils::add_val(&val, &b_value));
                }
            })}>{format!("+{}", value)}</button>

            if value % 3 == 0 {
                <br />
            }
        </>
    });
    
    html! {
        <>
            <h1>{"Calculator"}</h1>
            <div>
                { for buttons}
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
