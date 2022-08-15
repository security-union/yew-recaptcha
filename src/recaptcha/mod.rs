use gloo_console::{error, log};
use gloo_utils::{document, window};
use js_sys::{Function, Reflect};
use serde::Serialize;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::AddEventListenerOptions;
use yew::prelude::*;

#[derive(Serialize)]
pub struct RecaptchaAction {
    pub action: String,
}

#[derive(PartialEq, Properties)]
pub struct RecaptchaProps {
    pub site_key: String,
    pub on_execute: Option<Callback<String>>,
}

const GRECAPTCHA_DOM_ID: &str = "grecaptcha";
const GRECAPTCHA_URL: &str = "https://www.google.com/recaptcha/api.js";
const GRECAPTCHA_ON_LOAD: &str = "GoogleRecaptchaLoaded";

// Docs: https://developers.google.com/recaptcha/docs/v3
#[function_component(Recaptcha)]
pub fn recaptcha_component(props: &RecaptchaProps) -> Html {
    let site_key = props.site_key.clone();
    use_effect_with_deps(
        move |_| {
            if let Err(e) = inject_script(site_key.clone()) {
                error!(e);
            }
            || ()
        },
        (), // dependents
    );
    if let Some(callback) = &props.on_execute {
        if let Err(e) = execute(props.site_key.clone()) {
            error!(e);
        }
    }
    html! {
        <>
        </>
    }
}

fn execute(site_key: String) -> Result<(), JsValue> {
    let grecaptcha = window()
        .get(GRECAPTCHA_DOM_ID)
        .ok_or(JsValue::from_str("Can't find grecaptcha"))?;
    let grecaptcha: &wasm_bindgen::JsValue = &grecaptcha.into();
    let on_ready = Reflect::get(grecaptcha, &JsValue::from_str("ready"))?;
    let on_ready: Function = on_ready.into();
    let execute = Reflect::get(grecaptcha, &JsValue::from_str("execute"))?;
    let execute: Function = execute.into();
    let on_execute_callback = Closure::wrap(Box::new(move |token| {
        // callback.emit(token);
        log!("on_execute_callback {}", token);
    }) as Box<dyn FnMut(JsValue)>);
    let on_ready_callback = Closure::wrap(Box::new(move |_| {
        log!("on_ready");
        let action = &JsValue::from_serde(&RecaptchaAction {
            action: "submit".to_string(),
        })
        .unwrap();
        let future: js_sys::Promise = execute
            .call2(&JsValue::null(), &JsValue::from_str(&site_key), &action)
            .unwrap()
            .into();
        future.then(&on_execute_callback);
    }) as Box<dyn FnMut(JsValue)>);

    on_ready.call1(&JsValue::null(), &on_ready_callback.into_js_value())?;

    Ok(())
}

fn inject_script(site_key: String) -> Result<(), JsValue> {
    let google_loaded = Closure::wrap(Box::new(|_| {
        log!("loaded recaptcha");
    }) as Box<dyn FnMut(JsValue)>);

    Reflect::set(
        &JsValue::from(web_sys::window().unwrap()),
        &JsValue::from("GoogleRecaptchaLoaded"),
        google_loaded.as_ref().unchecked_ref(),
    )?;
    google_loaded.forget();
    let script = document().create_element("script").unwrap();
    script.set_attribute("async", "true")?;
    script.set_id(GRECAPTCHA_DOM_ID);
    let listener = Closure::wrap(Box::new(|_| {}) as Box<dyn FnMut(JsValue)>);
    let options = AddEventListenerOptions::new();
    script.add_event_listener_with_callback_and_add_event_listener_options(
        "onerror",
        listener.as_ref().unchecked_ref(),
        &options,
    )?;
    let site_url = format!(
        "{}?onload={}&render={}",
        GRECAPTCHA_URL, GRECAPTCHA_ON_LOAD, site_key
    );
    script.set_attribute("src", &site_url)?;

    script.set_attribute("type", "text/javascript")?;
    let body = document()
        .body()
        .ok_or(JsValue::from_str("Can't find body"))?;
    body.append_child(&script)?;
    Ok(())
}
