use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::molecules::{nav_bar::NavBar, contact_footer::ContactFooter};

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

#[styled_component(AboutMe)]
pub fn stargaze() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html!(
        <div class={stylesheet}>
            <div class={"about-me"}>
                <NavBar />
                <h1> {"About Me"} </h1>
                <div class={"content-container"}>
                    <p>{"Hello! I am a software developer with a profound enthusiasm for the world of technology and innovation. I'm a Biomedical Engineering graduate from The Johns Hopkins University, originally from Baltimore, Maryland, and now living in Miami, Florida. My academic journey equipped me with a diverse programming toolkit, including C, Java, Python, JavaScript, HTML, CSS, and Matlab. However, it was Rust's promise of performance and reliability that captured my professional dedication.

                    My passion for Rust is not just about the language itself, but about the potential it holds in areas that intrigue me the most: the precision of biotechnology, the dynamism of cryptocurrency markets, the challenge of mastering foreign languages, and the immersive experiences of video games. I approach learning with a voracious appetite, ever-eager to explore how the robustness of Rust can be leveraged to solve complex problems in these domains.
                    
                    As a developer, my goal is to intertwine my technical skills with my interests, creating solutions that are not only efficient and safe but also groundbreaking. With each line of code, I strive to bring a blend of engineering acumen and creative insight, driven by a belief that software is a pivotal tool in sculpting the innovations of tomorrow."}</p>
                    <img src={"img/developer-photo.jpeg"} alt={"A photograph of the developer"} />
                </div>
                <ContactFooter />
            </div>
        </div>
    )
}