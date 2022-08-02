use dioxus::prelude::*;

fn main() {
    env_logger::init();
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div {   
                background_color: "orange",
                h1    {"Interfacing sensor with USB."}
                p     {"Click on the buttons to have information from the board."}
        },
        
        button {
            onclick: move |evt| println!("I've been clicked!"),
            "click me!"
        }    ))
}
