use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct RecaptchaProps {
    pub site_key: String,
}

// Docs: https://developers.google.com/recaptcha/docs/v3
pub struct Recaptcha;

impl Component for Recaptcha {
    type Message = ();
    type Properties = RecaptchaProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            {
                format!(
                    "site_key: {}",
                    ctx.props().site_key,
                )
            }
        }
    }
}
