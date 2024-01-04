use yew::prelude::*;
use yew_router::prelude::*;

use crate::{cubes_traveller::CubesTraveller, peak_panic::PeakPanic};

#[derive(Clone, Routable, PartialEq)]

pub enum Route {
    #[at("/")]
    Home,

    #[at("/peak-panic-game")]
    PeakPanicGame,
    #[at("/cubes-traveller-game")]
    CubesTravellerGame,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
        <div class="flex flex-col">
            <Switch<Route> render={switch} />
        </div>
        </HashRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::PeakPanicGame => html! { <PeakPanic /> },
        Route::CubesTravellerGame => html! { <CubesTraveller /> },
    }
}

/// This is the home page of the website
/// This is where we will have the links töo the games,
/// the team, and the contact page
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <h1>{"Home"}</h1>
    }
}
