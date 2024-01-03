use gloo_console::{error, log};
use gloo_utils::{document, window};
use js_sys::{Function, JsString, Reflect};
use serde::Serialize;
use serde_wasm_bindgen::to_value;
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
pub fn use_recaptcha(
    site_key: String,
    on_execute: Box<UseStateHandle<Option<Callback<String>>>>,
) -> () {
    let key_clone = site_key.clone();
    use_effect_with(
        (),
        move |_| {
            if let Err(e) = inject_script(key_clone) {
                error!(e);
            }
            || ()
        },
    );

    // Only recompute if the on_execute callback is recomputed.
    let on_execute_clone = on_execute.clone();
    use_effect_with(
        *on_execute_clone,
        move |_| {
            if let Some(_callback) = &**on_execute.clone() {
                let future = execute(site_key, on_execute.clone());
                wasm_bindgen_futures::spawn_local(async move {
                    if let Err(e) = future.await {
                        error!(e);
                    }
                });
            }

            || ()
        },
    );
}

async fn execute(
    site_key: String,
    callback: Box<UseStateHandle<Option<Callback<String>>>>,
) -> Result<(), JsValue> {
    let callback = match &**callback {
        Some(callback) => callback,
        None => return Err(JsValue::from_str("No callback")),
    };
    let grecaptcha = window()
        .get(GRECAPTCHA_DOM_ID)
        .ok_or(JsValue::from_str("Can't find grecaptcha"))?;
    let grecaptcha: &wasm_bindgen::JsValue = &grecaptcha.into();
    let execute = Reflect::get(grecaptcha, &JsValue::from_str("execute"))?;
    let execute: Function = execute.into();
    let action = &to_value(&RecaptchaAction {
        action: "submit".to_string(),
    })
    .unwrap();
    let future: js_sys::Promise = execute
        .call2(&JsValue::null(), &JsValue::from_str(&site_key), &action)
        .unwrap()
        .into();
    let result = wasm_bindgen_futures::JsFuture::from(future).await?;
    let token = JsString::from(result)
        .as_string()
        .ok_or(JsValue::from_str("Can't parse recaptcha token"))?;
    callback.emit(token);
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
