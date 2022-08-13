use gloo_console::log;
use gloo_utils::document;
use js_sys::Reflect;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::AddEventListenerOptions;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct RecaptchaProps {
    pub site_key: String,
    pub on_execute: Option<Callback<String>>,
}

// Docs: https://developers.google.com/recaptcha/docs/v3
#[function_component(Recaptcha)]
pub fn recaptcha_component(props: &RecaptchaProps) -> Html {
    log!("cause child re-render");
    use_effect_with_deps(
        move |_| {
            log!("configuring dom");
            inject_script();
            || ()
        },
        (), // dependents
    );
    if let Some(callback) = &props.on_execute {
        log!("calling callback");
        callback.emit("balls".to_string());
    } else {
        log!("no callback");
    }
    html! {
        <>
        </>
    }
}

fn inject_script() -> Result<(), JsValue> {
    let google_loaded = Closure::wrap(Box::new(|_| {
        log!("loaded captcha");
    }) as Box<dyn FnMut(JsValue)>);

    Reflect::set(
        &JsValue::from(web_sys::window().unwrap()),
        &JsValue::from("GoogleRecaptchaLoaded"),
        google_loaded.as_ref().unchecked_ref(),
    )?;
    google_loaded.forget();
    let script = document().create_element("script").unwrap();
    script.set_attribute("async", "true")?;
    script.set_id("recaptcha");
    let listener = Closure::wrap(Box::new(|_| {}) as Box<dyn FnMut(JsValue)>);
    let options = AddEventListenerOptions::new();
    script.add_event_listener_with_callback_and_add_event_listener_options(
        "onerror",
        listener.as_ref().unchecked_ref(),
        &options,
    )?;
    script.set_attribute(
        "src",
        "https://www.google.com/recaptcha/api.js?onload=GoogleRecaptchaLoaded&render=explicit",
    )?;

    script.set_attribute("type", "text/javascript")?;

    // nonce && script.setAttribute("nonce", nonce);
    let body = document()
        .body()
        .ok_or(JsValue::from_str("Can't find body"))?;
    body.append_child(&script)?;
    Ok(())
}
