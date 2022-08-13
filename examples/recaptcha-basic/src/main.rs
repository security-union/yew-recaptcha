use gloo_console::log;
use yew::prelude::*;
use yew_recaptcha_v3::recaptcha::Recaptcha;

#[function_component(App)]
fn app_component() -> Html {
    let recaptcha_ref = NodeRef::default();
    let on_execute = Box::new(use_state(|| None));
    let on_execute_copy = on_execute.clone();
    let on_click = Callback::from(move |_| {
        log!("click");
        let on_execute_copy = on_execute.clone();
        on_execute.set(Some(Callback::from( move |w: String| {
            log!("on execute");
            on_execute_copy.set(None);
            ()
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
            site_key="6LeH_x8UAAAAAKKuaaod4GsENkTJTHdeQIm8l6y2"
            on_execute={on_execute_value}
            ref={recaptcha_ref}/>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
