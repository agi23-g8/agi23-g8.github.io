use yew::prelude::*;
use yew_router::prelude::*;

use crate::{app::Route, header::Header};

#[function_component(Team)]
pub fn team() -> Html {
    html! {
        <>
            <h1 class="text-5xl font-bold text-center">{"Team"}</h1>
            <div class="flex flex-row flex-grow flex-wrap justify-center items-start m-10 gap-10">
                <ProjectMember
                    name="David Åsberg"
                    image="/img/team/david.png"
                    badges={vec!["Graphics", "Interaction", "Game Design", "Networking", "Shaders", "Project Structure"]}
                    github="https://github.com/davidasberg"
                    linkedin="https://www.linkedin.com/in/david-aasberg/"
                    description="I have worked on many areas of both projects, but my main focus has been on the graphics and interaction. I have worked on the lighting, post-processing, and shaders. I have also worked on the interaction for each of the different controllers, and the networking to allow any mobile device to be used as a controller. Lastly, I have worked on the game design, and the overall structure of the projects."
                />

                <ProjectMember
                    name="Anders Blomqvist"
                    image="/img/team/anders.png"
                    badges={vec!["Graphics", "Github", "Level Design", "UI Design", "Networking", "Modeling", "Game Design"]}
                    github="https://github.com/andersblomqvist"
                    linkedin="https://www.linkedin.com/in/anders-blomqvist-9b7958143/"
                    description="For Cube's Traveller, I contributed to various aspects of the project, but my primary focus was on graphics and 3D modeling. In the project's early stages, I established a GitHub workflow and created the initial gameplay iteration. I also developed the sky shader, including the day and night cycle, and created most of our 3D models. Additionally, I have made sure that our game looks good from a visual standpoint where I provided feedback to colleagues and parameter tuning.\n\nFor Peak Panic, I was involved in many different parts of our project. I began with creating the first prototype which showcased the idea of controlling a player with the accelerometer. From this, I suggested a skiing game in which we could reuse some assets from our previous project. My focus was implementing the WebGL client but also 3D modeling together with designing the level. I created all models except two, the player character and the snow canon. In the later stages of the project, I closed the gameplay loop and provided feedback on our camera system. Additionally, I spent some time making nice graphics and a title screen – making the project feel complete."
                />

                <ProjectMember
                    name="Gabriel Françon"
                    image="/img/team/gabriel.png"
                    badges={vec!["Graphics", "Shader Dev", "Texturing", "Level Art", "Gameplay", "Blender"]}
                    github="https://github.com/GabFrancon"
                    linkedin="https://www.linkedin.com/in/gabriel-francon/"
                    description="Focused on enhancing visual experiences, I crafted versatile custom shaders encompassing splat map-based ground texturing, procedural grass generation, and realistic snow deformation. Additionally, I handled terrain modeling and orchestrated the level art for both projects, effectively showcasing these shaders. In terms of gameplay, I engineered a physics-based skier controller and directed a path-based camera to ensure smooth multiplayer experience. My involvement extended to collaborative efforts in fine-tuning scene rendering and actively contributing to the documentation of our work on the website."
                />

                <ProjectMember
                    name="Jean-Louis Werthe"
                    image="/img/team/jean-louis.png"
                    badges={vec!["Blender", "Modeling", "Graphics", "Animation"]}
                    description="I have acquired skills in Blender, including model creation, rigging, and animation. I have also developed a good understanding of Unity, enabling me to incorporate properties and implement control scripts on objects effectively. Furthermore, I have delved into how to effectively interface Unity with Blender, ensuring a smooth integration of Blender data."
                />

                <ProjectMember
                    name="Jérémy Carneau"
                    image="/img/team/jeremy.jpg"
                    badges={vec!["Gameplay", "Game Design", "Modeling", "Interaction"]}
                    github="https://github.com/Jeremy-Carneau"
                    linkedin="https://linkedin.com/in/jeremy-carneau"
                    description="I contributed modestly to several aspects of the projects. I contributed to the development of the first versions of Cube's Traveller's interaction system, which managed the cube's rotation using the phone's gyroscope. I also started working on puzzles for a future level that never saw the light of day. I also concentrated on the gameplay of Peak Panic, adding elements such as boosts and snow cannons to bring the race to life."
                />
            </div>
        </>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct ProjectMemberProps {
    name: AttrValue,
    image: AttrValue,
    badges: Vec<&'static str>,
    github: Option<AttrValue>,
    linkedin: Option<AttrValue>,
    description: AttrValue,
}

#[function_component(ProjectMember)]
fn project_member(props: &ProjectMemberProps) -> Html {
    let ProjectMemberProps {
        name,
        image,
        badges,
        github,
        linkedin,
        description,
    } = props.clone();

    let badges = badges
        .into_iter()
        .map(|badge| html! { <span class="bg-gray-100 text-gray-800 text-sm font-medium me-2 px-2.5 py-0.5 rounded dark:bg-gray-700 dark:text-gray-300 text-center">{badge}</span> })
        .collect::<Html>();

    html! {
        <div class="flex flex-col items-center gap-2 max-w-md ">
            <img class="object-cover rounded-full w-40 h-40" src={image} alt={name.clone()} />
            <p class="text-2xl font-bold">{name}</p>
            <div class="flex flex-row gap-2">
                {github.map(|link| html! {
                    <a href={link}>
                        <span class="bg-green-100 text-green-800 text-sm font-medium me-2 px-2.5 py-0.5 rounded dark:bg-green-900 dark:text-green-300">{"Github"}</span>
                    </a>
                })}
                {linkedin.map(|link| html! {
                    <a href={link}>
                        <span class="bg-blue-100 text-blue-800 text-sm font-medium me-2 px-2.5 py-0.5 rounded dark:bg-blue-900 dark:text-blue-300">{"LinkedIn"}</span>
                    </a>
                })}
            </div>

            <div class="flex flex-row flex-grow flex-wrap gap-2 justify-center">
                {badges}
            </div>

            <p class="text-justify max-w-prose">{description}</p>
        </div>
    }
}
