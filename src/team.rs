use yew::prelude::*;
use yew_router::prelude::*;

use crate::{app::Route, header::Header};

#[function_component(Team)]
pub fn team() -> Html {
    html! {
        <>
            <Header />
            <p>{"This is the team page."}</p>
        </>
    }
}
