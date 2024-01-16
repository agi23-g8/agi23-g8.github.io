//! Module for our Peak Panic game

use yew::prelude::*;
use yew_router::prelude::*;

use crate::{carousel::Carousel, gallery::Gallery};

/// This is the page for our Peak Panic game
/// Its a portolio page for the game
/// It will contain a description of the game
/// and images, videos
#[function_component(CubesTraveller)]
pub fn cubes_traveller() -> Html {
    let images = vec![
        "img/cubes_traveller/game/splash.png",
        "img/cubes_traveller/game/cloud.jpg",
        "img/cubes_traveller/game/dusk.jpg",
        "img/cubes_traveller/game/night.jpg",
        "img/cubes_traveller/game/pumpkin_night.png",
        "img/cubes_traveller/game/leaning.jpg",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<String>>();

    let user_images = vec![
        "img/cubes_traveller/users/ct_user_testing_1.png",
        "img/cubes_traveller/users/ct_user_testing_2.png",
        "img/cubes_traveller/users/ct_user_testing_3.png",
        "img/cubes_traveller/users/ct_user_testing_4.png",
        "img/cubes_traveller/users/ct_user_testing_5.png",
        "img/cubes_traveller/users/ct_user_testing_6.png",
        "img/cubes_traveller/users/ct_user_testing_7.png",
        "img/cubes_traveller/users/ct_user_testing_8.png",
        "img/cubes_traveller/users/ct_user_testing_9.png",
        "img/cubes_traveller/users/ct_user_testing_10.png",
        "img/cubes_traveller/users/ct_user_testing_11.png",
        "img/cubes_traveller/users/ct_user_testing_12.png",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<String>>();

    html! {
        <>
        <div class="container mx-auto flex flex-col my-6 px-4 sm:px-6 lg:px-8 items-center justify-items-center">
            <h1 class="text-4xl my-5 font-extrabold text-center tracking-tight sm:text-5xl lg:text-8xl">{"Cube's Traveller"}</h1>
            <p class="text-xl my-3 text-center tracking-tight">{"A cozy, cooperative puzzle game for two players"}</p>
            // Image of the game
            <figure class="flex flex-col items-center">
                <img class="rounded-lg shadow-lg" src="img/cubes_traveller/game/splash.png" alt="Cube's Traveller" />
            </figure>

            <h2 class="text-2xl my-5 font-extrabold text-center tracking-tight sm:text-3xl lg:text-5xl">{"A new way to play together"}</h2>

            <p class="text-xl my-3 text-justify max-w-prose tracking-tight">{"Cube’s Traveller is a cooperative game where you are supposed to work together in an environment which is visually stunning, but challenges your world view. In this game, one player is controlling the world, which is a cube, and the other player controls a character on the cube. Our goal is to create a new interactive experience without the often disappointing experiences of VR/AR, and instead focus on different interaction technologies. Additionally, a major goal of our game is to make it visually beautiful to help players immerse themselves into the game world. We thought that previous projects have lacked the visual fidelity we want to achieve."}</p>

            <p class="text-xl my-3 text-justify max-w-prose tracking-tight">{"The motivation for a cooperative game was simple, we think that playing games with a friend is really fun, therefore it was very obvious to select it as the main project focus. With the limited time and ambitious graphical goals, we limited ourselves to a simple, yet effective, art style where models and textures are basic but lighting is complex and beautiful. We believe that this helped us stand out from the rest and create a unique experience."}</p>

            <Gallery images={images} />

            // Trailer
            <div class="w-full my-10">
                <iframe class="w-full" height="720" src="https://www.youtube.com/embed/nRWKN7nUR1c?si=rrIReJi69XXHtMy1" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen=true></iframe>
            </div>

            // Making Of
            <div class="w-full my-10">
                <iframe class="w-full" height="720" src="https://www.youtube.com/embed/RiNYSEQuXiY" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen=true></iframe>
            </div>

            // Text about the game
            <div class="lg:columns-2 gap-10">
                <div class="break-inside-avoid-column">
                <h2 class="text-2xl my-5 font-extrabold tracking-tight sm:text-3xl lg:text-5xl">{"Interaction"}</h2>
                <p class="text-xl my-3 text-justify max-w-prose tracking-tight">
                    {"One of the users controls the ingame cube by rotating the phone on all axes. We use an android phone that runs an app, which we built and downloaded to the phone. The app is running an instance of Unity which is sampling the Gyro input controller. Additionally, we use Unity’s netcode for game objects in order to send the gyroscope data to the game server, which can then update the cube’s rotation. We could have used an iPhone as well but that requires additional Apple hardware, such as a Mac computer, and building it through XCode. We did not have access to any Apple hardware."}
                    <br/>
                    <br/>

                    {"The networking solution is pretty straight forward: the game sets up a server on the local network and broadcasts a message saying: “hello I am a server!”. Then, the phone, which is on the same local network as the server, simply looks for any broadcasted messages and attempts to establish a connection. After a connection is made, we can start sending data on the local network."}
                    <br/>
                    <br/>
                    {"The other user controls a character, which is walking on the cube - as if it was a planet. For this player, we use a Xbox controller where, for example, holding the left stick upwards will make the player go up relative to the camera view. It was necessary to make it relative to the camera space because the camera is static and the whole world moves. Otherwise it was incredibly hard to navigate on the cube’s faces. This makes it so up is always “up” and users who tested it felt the controls were intuitive - which is great!"}
                    <br/>
                    <br/>

                    {"Additionally, the goal of the game is to get a ball from one side of the cube to the opposite. We tested different types of ball mechanics, such as different gravity behavior. First we had real world gravity for the ball, i.e the ball would fall down relative to the camera. However, we later settled on making the ball have the same gravity model as the player, which is being stuck on the cube like a planet. From the feedback we got, this made more sense and made the experience coherent and predictable."}
                </p>
                </div>

                <div class="break-inside-avoid-column">
                <h2 class="text-2xl my-5 font-extrabold tracking-tight sm:text-3xl lg:text-5xl">{"Graphics"}</h2>
                <p class="text-xl my-3 text-justify max-w-prose tracking-tight">
                    {"Early in our project planning, we made a deliberate choice to prioritize graphics using the Unity engine, given the keen interest of a substantial part of our team in advancing within this realm. Our primary objective was to incorporate cutting-edge techniques, particularly those associated with lighting and volumetric effects, while still keeping our 3D assets simple. This dual approach of using low-poly models illuminated by sophisticated lighting effects was designed to confer a unique and memorable visual identity to our prototype."}
                    <br/>
                    <br/>

                    {"In order to lend depth and realism to the terrain of our rotating cube, especially in the grassy areas of the first level, we aimed to introduce three-dimensional grass blades with high density, all without the need for manual placement. Our solution was a procedural shader, designed to autonomously generate the geometry of grass blades in specific regions of the cube. This shader also manages the lighting and shadowing of the grass, ensuring a seamless integration into the rendering pipeline"}
                    <br/>
                    <br/>

                    {"Furthermore, we sought to create a relaxing day and night cycle by incorporating a dynamic grading of the skycolor, and multiple directional light sources (sun and moon). In addition to the dynamic sky grading, we incorporated fluffy clouds in the sky to enhance the background. Although their position remains fixed, their vertices are animated over time, lending
                    them a unique non-realistic style that aligns with our stylized artistic direction. This approach contributed to the overall aesthetic and atmosphere of the game."}
                    <br/>
                    <br/>

                    {"Our game was already bathed in captivating lighting, but we aspired to enhance it further by introducing a subtle volumetric effect that would beautifully capture the interplay of sun and moon rays with the ambient fog and swaying grass blades. To realize this vision, we embarked on another custom shader journey, this time orchestrating multiple screen-space passes to achieve the desired volumetric effects"}
                    <br/>
                    <br/>

                    {"Towards the culmination of our graphics pipeline, we harnessed a suite of essential post-processing effects, wielding the power to significantly enhance the visual appeal of our scene. The Universal Render Pipeline furnished us with a collection of shaders, simplifying the implementation and customization of these common post processes. Several noteworthy effects were incorporated to refine our scene"}
                </p>
                </div>

                <div class="h-1"/> // Absolutely disgusting, but it works
                <div class="break-inside-avoid-column">
                <h2 class="text-2xl my-5 font-extrabold tracking-tight sm:text-3xl lg:text-5xl">{"Challenges and obstacles"}</h2>
                <p class="text-xl my-3 text-justify max-w-prose tracking-tight">
                    {"One challenge for this project was creating the gameplay. It was not obvious on how to design each player’s role and purpose. We had to experiment and iterate what type of gameplay would make sense. We managed to settle on a football type of game after several rounds of user feedback."}
                    <br/>
                    <br/>

                    {"An issue we had to deal with during later stages of our project was performance. We have an unconventional setup where the whole world is moving together with complex graphical features and lighting. Due to the movable world, no objects could be “static”, which means they cannot be taken into account for baked lighting. Therefore, all lighting and shadows are dynamic and calculated in real time. The performance issue arose during the project’s last half due to the fact that we had been too generous with the number of assets placed around the cube planet. However, fixing this issue was simple: remove unnecessary assets and select a few key assets which should cast a shadow. For example, all small flowers, rocks, and other minor detail assets do not have shadows, while other bigger assets, such as trees, cast shadows."}
                </p>
                </div>

                <div class="break-inside-avoid-column">
                <h2 class="text-2xl my-5 font-extrabold tracking-tight sm:text-3xl lg:text-5xl">{"Related work"}</h2>
                <p class="text-xl my-3 text-justify max-w-prose tracking-tight">
                    {"Our cozy cooperative game draws inspiration from diverse sources, each contributing unique elements to our project. Fez (2012) captivates us with its voxel design and artistic direction, influencing our visual aesthetics. The game's clever manipulation of camera perspectives to impact mechanics has inspired our approach to interactive gameplay."}
                    <br/>
                    <br/>
        
                    {"In the realm of cube-based environments, Moncage (2021) serves as a significant influence. Within its mysterious cube, distinct worlds reside on each side. Aligning these worlds through precise camera placement unlocks new game mechanics and offers stunning visuals. Moncage's innovative use of the cube concept resonates with our vision for an engaging and dynamic game environment."}
                    <br/>
                    <br/>
        
                    {"Adding an interactive dimension to our project is the Merge Cube, originally developed for educational purposes. This physical cube can be hand-manipulated in all directions, influencing the virtual scene projected onto it through augmented reality via a phone screen. The Merge Cube's fusion of the physical and virtual worlds has inspired our phone-based cube gameplay mechanics."}
                </p>
                </div>

                <div class="break-inside-avoid-column">
                <h2 class="text-2xl my-5 font-extrabold tracking-tight sm:text-3xl lg:text-5xl">{"Lessons Learned"}</h2>
                <p class="text-xl my-3 text-justify max-w-prose tracking-tight">
                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                    <br/>
                    <br/>

                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                </p>
                </div>
            </div>

            <h2 class="text-2xl pt-10 font-extrabold tracking-tight sm:text-3xl lg:text-5xl">{"User Testing"}</h2>
            <Gallery images={user_images} />
        </div>


        </>
    }
}
