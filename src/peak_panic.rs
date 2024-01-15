//! Module for our Peak Panic game

use yew::prelude::*;
use yew_router::prelude::*;

use crate::gallery::Gallery;

/// This is the page for our Peak Panic game
/// Its a portolio page for the game
/// It will contain a description of the game
/// and images, videos
#[function_component(PeakPanic)]
pub fn peak_panic() -> Html {
    let images = vec![
        "img/peak_panic/pp_environment_1.jpg".to_string(),
        "img/peak_panic/pp_environment_2.jpg".to_string(),
        "img/peak_panic/pp_environment_3.jpg".to_string(),
        "img/peak_panic/pp_environment_4.jpg".to_string(),
        "img/peak_panic/pp_environment_5.jpg".to_string(),
        "img/peak_panic/pp_gameplay_1.png".to_string(),
        "img/peak_panic/pp_gameplay_2.png".to_string(),
        "img/peak_panic/splash.png".to_string(),
    ];

    html! {
        <>
        <div class="container mx-auto flex flex-col my-6 px-4 sm:px-6 lg:px-8 items-center justify-items-center">
            <h1 class="text-4xl my-5 font-extrabold text-center tracking-tight sm:text-5xl lg:text-8xl">{"Peak Panic"}</h1>
            <p class="text-xl font-light my-3 text-center tracking-tight">{"An easily accessible, fast-paced, and fun racing game where you compete against your friends"}</p>
            // Image of the game
            <figure class="flex flex-col items-center">
                <img class="rounded-lg shadow-lg" src="img/peak_panic/splash.png" alt="Cube's Traveller" />
            </figure>

            <h2 class="text-2xl my-5 font-extrabold text-center tracking-tight sm:text-3xl lg:text-5xl">{"A new way to play together"}</h2>

            <p class="text-xl font-light my-3 text-justify max-w-prose tracking-tight">{"Peak Panic is a local multiplayer skiing game where you have to keep up the pace so you don't fall behind! Each player controls a character using their own phone by tilting it left and right."}</p>

            <p class="text-xl font-light my-3 text-justify max-w-prose tracking-tight">{"A player is eliminated when going out of view of the camera. The camera will follow the leading player with a slight delay for letting players behind catch up. The slope is filled with obstacles such as trees and rolling snowballs that you have to avoid. Additionally, there are speed boosts placed around the slope that a player can pick up in order to gain an advantage. Let the fastest skier win!"}</p>

            <Gallery images={images} />

            <div class="w-full my-10">
                <iframe class="w-full" height="720" src="https://www.youtube.com/embed/bw2AXm7WpNk?si=jdedPxbdXF9k8MEk" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen=true></iframe>
            </div>

            // Text about the game
            <div class="lg:columns-2 gap-10">
                <div class="break-inside-avoid-column">
                <h2 class="text-2xl my-5 font-extrabold tracking-tight sm:text-3xl lg:text-5xl">{"Interaction"}</h2>
                <p class="text-xl font-light my-3 text-justify max-w-prose tracking-tight">
                    {"One of the users controls the ingame cube by rotating the phone on all axes. We use an android phone that runs an app, which we built and downloaded to the phone. The app is running an instance of Unity which is sampling the Gyro input controller. Additionally, we use Unity’s netcode for game objects in order to send the gyroscope data to the game server, which can then update the cube’s rotation. We could have used an iPhone as well but that requires additional Apple hardware, such as a Mac computer, and building it through XCode. We did not have access to any Apple hardware."}
                    <br/>
                    {"The networking solution is pretty straight forward: the game sets up a server on the local network and broadcasts a message saying: “hello I am a server!”. Then, the phone, which is on the same local network as the server, simply looks for any broadcasted messages and attempts to establish a connection. After a connection is made, we can start sending data on the local network."}
                    <br/>
                    {"The other user controls a character, which is walking on the cube - as if it was a planet. For this player, we use a Xbox controller where, for example, holding the left stick upwards will make the player go up relative to the camera view. It was necessary to make it relative to the camera space because the camera is static and the whole world moves. Otherwise it was incredibly hard to navigate on the cube’s faces. This makes it so up is always “up” and users who tested it felt the controls were intuitive - which is great!"}
                    <br/>
                    {"Additionally, the goal of the game is to get a ball from one side of the cube to the opposite. We tested different types of ball mechanics, such as different gravity behavior. First we had real world gravity for the ball, i.e the ball would fall down relative to the camera. However, we later settled on making the ball have the same gravity model as the player, which is being stuck on the cube like a planet. From the feedback we got, this made more sense and made the experience coherent and predictable."}
                </p>
                </div>

                <div class="break-inside-avoid-column">
                <h2 class="text-2xl my-5 font-extrabold tracking-tight sm:text-3xl lg:text-5xl">{"Graphics"}</h2>
                <p class="text-xl font-light my-3 text-justify max-w-prose tracking-tight">
                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                    <br/>
                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                    <br/>
                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                    <br/>
                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                </p>
                </div>

                <div class="break-inside-avoid-column">
                <h2 class="text-2xl my-5 font-extrabold tracking-tight sm:text-3xl lg:text-5xl">{"Challenges and obstacles"}</h2>
                <p class="text-xl font-light my-3 text-justify max-w-prose tracking-tight">
                    {"One challenge for this project was creating the gameplay. It was not obvious on how to design each player’s role and purpose. We had to experiment and iterate what type of gameplay would make sense. We managed to settle on a football type of game after several rounds of user feedback."}
                    <br/>
                    {"An issue we had to deal with during later stages of our project was performance. We have an unconventional setup where the whole world is moving together with complex graphical features and lighting. Due to the movable world, no objects could be “static”, which means they cannot be taken into account for baked lighting. Therefore, all lighting and shadows are dynamic and calculated in real time. The performance issue arose during the project’s last half due to the fact that we had been too generous with the number of assets placed around the cube planet. However, fixing this issue was simple: remove unnecessary assets and select a few key assets which should cast a shadow. For example, all small flowers, rocks, and other minor detail assets do not have shadows, while other bigger assets, such as trees, cast shadows."}
                </p>
                </div>

                <div class="break-inside-avoid-column">
                <h2 class="text-2xl my-5 font-extrabold tracking-tight sm:text-3xl lg:text-5xl">{"Related work"}</h2>
                <p class="text-xl font-light my-3 text-justify max-w-prose tracking-tight">
                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                    <br/>
                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                </p>
                </div>

                <div class="break-inside-avoid-column">
                <h2 class="text-2xl my-5 font-extrabold tracking-tight sm:text-3xl lg:text-5xl">{"Lessons Learned"}</h2>
                <p class="text-xl font-light my-3 text-justify max-w-prose tracking-tight">
                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                    <br/>
                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                </p>
                </div>
            </div>
        </div>


        </>
    }
}
