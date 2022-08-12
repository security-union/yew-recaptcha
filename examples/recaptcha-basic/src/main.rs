use gloo_console::log;
use yew::prelude::*;
use yew_recaptcha_v3::recaptcha::Recaptcha;

#[function_component(App)]
fn app_component() -> Html {
    let recaptcha_ref = NodeRef::default();
    let on_click = Callback::from(|_| {
        log!("click");
        ()
    });

    html! {
        <>
            <button onclick={on_click}>
                { "Click me!" }
            </button>
            <Recaptcha
            site_key="6LeH_x8UAAAAAKKuaaod4GsENkTJTHdeQIm8l6y2"
            // onResolved={onResolved}
            ref={recaptcha_ref}/>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
