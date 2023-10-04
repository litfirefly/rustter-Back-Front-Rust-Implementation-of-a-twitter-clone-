#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use fermi::use_init_atom_root;

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);
    let other_things = rsx! { p { "hi"} };
    cx.render(rsx! {
        Router {
            Route { to: page::ACCOUNT_REGISTER, page::Register { } },

        }
    })
}
