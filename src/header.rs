use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(Header)]
pub fn header() -> Html {
    let navigator = use_navigator().unwrap();

    let home_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Home));
        html! {
            <button {onclick} class="btn btn-primary normal-case text-xl transition delay-50 ease-in-out duration-150 hover:scale-110">{"Home"}</button>
        }
    };

    let team_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Team));
        html! {
            <button {onclick} class="btn btn-primary normal-case text-xl transition delay-50 ease-in-out duration-150 hover:scale-110">{"Team"}</button>
        }
    };

    html! {
        <div class="navbar bg-base-100">

            <div class="navbar-start flex gap-4">
                {home_button}
                {team_button}
            </div>

            <div class="navbar-center">
                <p class="text-white text-3xl font-bold">{"AGI 23 Group 8"}</p>
            </div>


            <div class="navbar-end"/>
        </div>
    }
}
