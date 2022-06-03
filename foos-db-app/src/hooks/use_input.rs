use web_sys::HtmlInputElement;
use yew::prelude::*;

#[hook]
pub fn use_input<F: FnOnce() -> String>(
    init_fn: F,
) -> (UseStateHandle<String>, Callback<InputEvent>) {
    let handle = use_state(init_fn);

    let callback = {
        let handle = handle.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_unchecked_into::<HtmlInputElement>();
            handle.set(target.value());
        })
    };

    (handle, callback)
}
