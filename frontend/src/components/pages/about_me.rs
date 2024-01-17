//! Webpage that describes the developer.

use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::{
    pages::scroll_to_top,
    subcomponents::{contact_footer::ContactFooter, nav_bar::NavBar},
};

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

/// The page of the web application that describes the developer.
#[styled_component(AboutMe)]
pub fn about_me() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    // Scroll to top of page on load
    scroll_to_top();

    html!(
        <div class={stylesheet}>
            <div class={"about-me"}>
                <NavBar />
                <div class={"content-container"}>
                    <section>
                        <p>{"My name is Xavier Griffith and I am a software developer with a profound enthusiasm for the world of technology, innovation, and pushing the boundaries of imagination."}</p>

                        <p>{"Originally from Baltimore, Maryland, I'm a Biomedical Engineering graduate from The Johns Hopkins University, currently living in Miami, Florida. My academic journey equipped me with a diverse problem-solving toolkit, including experience programming in C, Java, Python, JavaScript, HTML, CSS, and Matlab. However, it was only after graduating that I developed a deep interest for the Rust programming language."}</p>

                        <p>{"Beyond its syntax, I'm drawn to Rust's rich ecosystem, particularly its community-driven crates that fuel my exploration in cutting-edge fields like game development with Bevy, blockchain innovation through Solana, and seamless artificial intelligence integration using the OpenAI API by way of the async-openai crate. I approach learning with a voracious appetite, ever-eager to explore how the robustness of Rust can be leveraged to solve complex problems in these domains."}</p>

                        <p>{"As a developer, my goal is to intertwine my technical skills with my passions, creating solutions that are not only efficient and safe but also groundbreaking. With each line of code, I strive to bring a blend of engineering acumen and creative insight, driven by a belief that software is a pivotal tool in sculpting the innovations of tomorrow."}</p>
                    </section>
                    <img src={"img/developer-photo.jpeg"} alt={"A photograph of the developer"} />
                </div>
                <ContactFooter />
            </div>
        </div>
    )
}
