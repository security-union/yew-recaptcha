use gloo_console::log;
use yew::prelude::*;
use yew_recaptcha_v3::recaptcha::Recaptcha;

#[function_component(App)]
fn app_component() -> Html {
    let on_execute = Box::new(use_state(|| None));
    let on_execute_copy = on_execute.clone();
    let on_click = Callback::from(move |_| {
        log!("Button clicked");
        // Per https://yew.rs/docs/next/concepts/function-components/communication
        // We need to create a new callback everytime that we want Recaptcha to be executed.
        on_execute.set(Some(Callback::from(|token| {
            log!("on_execute_callback {}", token);
        })));
        ()
    });
    let on_execute_value = &**on_execute_copy;
    html! {
        <>
            <button onclick={on_click}>
                { "Click me!" }
            </button>
            <Recaptcha
            site_key="6LddvmMhAAAAAKeASefVl3YcOuM-sptuZ2Hmr0n1"
            on_execute={on_execute_value}
        />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
