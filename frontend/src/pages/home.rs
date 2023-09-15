use yew::prelude::*;
use stylist::{style, yew::styled_component};

// Displays the website homepage
#[styled_component(Home)]
pub fn home() -> Html {
    let stylesheet = style!(
        r#"
            header {
                text-align: center;
                padding: 50px;
            }
            
            header h1 {
                font-size: 2.5em;
                color: #a056f3;
            }
            
            header p {
                font-size: 1.5em;
                color: #08f7be;
            }
            
            section {
                margin: 50px;
                padding: 20px;
                border: 1px solid #a056f3;
            }
            
            section h2 {
                font-size: 2em;
                color: #08f7be;
            }
            
            section p {
                font-size: 1.25em;
            }
            
            section ul {
                list-style-type: none;
            }
            
            section ul li {
                font-size: 1.25em;
                margin: 10px 0;
            }
            
            section ul li a {
                color: #a056f3;
                text-decoration: none;
            }
            
            section ul li a:hover {
                text-decoration: underline;
            }
            
            footer {
                text-align: center;
                padding: 20px;
                border-top: 1px solid #a056f3;
            }
            
            footer p {
                margin: 10px 0;
            }
            
            footer p a {
                color: #08f7be;
            }
            
            footer p a:hover {
                text-decoration: underline;
            }
        "#
    ).unwrap();

    html!(
        <div class={stylesheet}>
        <header>
        <h1>{"Xavier Griffith"}</h1>
        <p>{"Specializing in Rust Development"}</p>
    </header>
    
    <section>
        <h2>{"About Me"}</h2>
        <p>{"I am a software developer proficient in the Rust programming language."}</p>
    </section>
    
    <section>
        <h2>{"Projects"}</h2>
        <ul>
            <li><a href="project1_url">{"Project 1"}</a></li>
            <li><a href="project2_url">{"Project 2"}</a></li>
            <li><a href="project3_url">{"Project 3"}</a></li>
        </ul>
    </section>
    
    <footer>
        <p>{"Contact: "}<a href="mailto:your.email@domain.com">{"xavierorlandogriffith@gmail.com"}</a></p>
        <p>{"GitHub: "}<a href="https://github.com/zayman2045">{"zayman2045"}</a></p>
    </footer>
        </div>
    )
}
