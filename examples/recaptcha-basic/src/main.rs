use gloo_console::log;
use yew::prelude::*;
use yew_recaptcha_v3::recaptcha::use_recaptcha;

const RECAPTCHA_SITE_KEY: &str = std::env!("RECAPTCHA_SITE_KEY");

#[function_component(App)]
fn app_component() -> Html {
    let last_token = use_state(|| None);
    let on_execute = use_state(|| None);
    let on_click = {
        let on_execute = on_execute.clone();
        let last_token = last_token.clone();
        Callback::from(move |_| {
            log!("Button clicked");
            let last_token = last_token.clone();
            
            // setting the on_execute callback will force recaptcha to be recalculated.
            on_execute.set(Some(Callback::from(move |token| {
                last_token.set(Some(token));
            })));
            ()
        })
    };
    let counter = use_state(|| 0);
    let on_counter_click = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    // Recaptcha will be called only when on_execute changes.
    use_recaptcha(RECAPTCHA_SITE_KEY.to_string(), on_execute);

    let print_last_token = match &(*last_token) {
        Some(token) => format!("reCAPTCHA token: {}", token),
        None => "Press the button to get a token, look at the console logs in case that there's an error".to_string()
    };
    html! {
        <>
            <button onclick={on_click}>
                { "Click me to call recaptcha!" }
            </button>
            <p>{print_last_token}</p>
            <p>{"Remember that you need to send this token along with the form values so that \n
                   your server can call the Recaptcha API."}</p>
            <p>{"The reCAPTCHA v3 returns a score (1.0 is very likely a good interaction,\
                   0.0 is very likely a bot). Based on the score, you can take variable action in\
                   the context of your site\
                   Take action behind the scenes instead of blocking traffic to \
                   better protect your site."
                }
            </p>
            <a href="https://developers.google.com/recaptcha/docs/v3">{"Google Documentation"}</a>
            <p>{"This counter is just to test that clicking this button does not cause recaptcha \n
                to be called"
            }</p>
            <button onclick={on_counter_click}>{ "+1" }</button>
            <p>{ *counter }</p>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
