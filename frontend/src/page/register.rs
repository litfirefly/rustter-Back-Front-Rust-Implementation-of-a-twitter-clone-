#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;

pub struct PageState {
    username: UseState<String>,
    password: UseState<String>,
}

impl PageState {
    pub fn new(cx: Scope) -> Self {
        Self {
            username: use_state(cx, String::new).clone(),
            password: use_state(cx, String::new).clone(),
        }
    }
}

#[inline_props]
pub fn UsernameInput<'a>(
    cx: Scope<'a>,
    state: UseState<String>,
    oninput: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex flex-col",
            label {
                r#for: "username",
                "Username"
            },
            input {
                id: "username",
                class: "input-field",
                name:"username",
                placeholder: "Username",
                value: "{state.current()}",
                oninput: move |ev| oninput.call(ev)
            }
        }
    })
}

#[inline_props]
pub fn PasswordInput<'a>(
    cx: Scope<'a>,
    state: UseState<String>,
    oninput: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex flex-col",
            label {
                r#for: "password",
                "Password"
            },
            input {
                id: "password",
                class: "input-field",
                name:"password",
                placeholder: "Password",
                value: "{state.current()}",
                r#type: "password",
                oninput: move |ev| oninput.call(ev)
            }
        }
    })
}

pub fn Register(cx: Scope) -> Element {
    let page_state = PageState::new(cx);
    let page_state = use_ref(cx, || page_state);

    let username_oninput = sync_handler!([page_state], move |ev: FormEvent| {
        page_state.with_mut(|state| state.username.set(ev.value.clone()))
    });

    let password_oninput = sync_handler!([page_state], move |ev: FormEvent| {
        page_state.with_mut(|state| state.password.set(ev.value.clone()))
    });

    cx.render(rsx! {
        form {
            class: "flex flex-col gap-5 m-5",
            prevent_default: "onsubmit",
            onsubmit: move |_| {},
            UsernameInput {
                state: page_state.with(|state|state.username.clone()),
                oninput:username_oninput
            }

            PasswordInput {
                state: page_state.with(|state|state.password.clone()),
                oninput: password_oninput
            }

            button {
                class: "btn",
                r#type:"submit",
                "Signup"
            }
        }
    })
}
