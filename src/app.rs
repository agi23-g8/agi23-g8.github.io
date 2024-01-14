use yew::prelude::*;
use yew_router::prelude::*;

use crate::{cubes_traveller::CubesTraveller, header::Header, peak_panic::PeakPanic, team::Team};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/team")]
    Team,
    #[at("/peak-panic-game")]
    PeakPanicGame,
    #[at("/cubes-traveller-game")]
    CubesTravellerGame,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
        <Switch<Route> render={switch} />
        </HashRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Team => html! { <Team /> },
        Route::PeakPanicGame => html! { <PeakPanic /> },
        Route::CubesTravellerGame => html! { <CubesTraveller /> },
    }
}

/// This is the home page of the website
/// This is where we will have the links töo the games,
/// the team, and the contact page
#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let peak_panic = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::PeakPanicGame))
    };

    let cubes_traveller = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::CubesTravellerGame))
    };
    html! {
        <>
        <Header />
        <div class="h-screen flex flex-col m-10 gap-10 xl:flex-row place-content-center">
            <div class="flex flex-grow flex-1 rounded-lg justify-center items-center transition delay-50 ease-in-out duration-150 hover:opacity-60 hover:cursor-pointer" onclick={peak_panic} style="background-image: url('/img/peak_panic/splash.png'); background-size: cover; background-position: center;">
                <p class="text-white text-3xl font-bold">{"Peak Panic"}</p>
            </div>

            <div class="flex flex-grow flex-1 rounded-lg justify-center items-center transition delay-50 ease-in-out duration-150 hover:opacity-60 hover:cursor-pointer" onclick={cubes_traveller} style="background-image: url('/img/cubes_traveller/game/splash.png'); background-size: cover; background-position: center;">
                <p class="text-white text-3xl font-bold">{"Cube's Traveller"}</p>
            </div>
        </div>

        <Team />

        </>


    }
}
