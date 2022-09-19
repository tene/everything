#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(app);
}

#[derive(Props)]
struct UserStatusProps<'a> {
    name: &'a str,
}

fn UserStatus<'a>(cx: Scope<'a, UserStatusProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            "{cx.props.name}"
        }
    })
}

fn Users(cx: Scope) -> Element {
    let my_fake_users = ["Alice", "Bob"];
    cx.render(rsx! {
        ul {
            class: "list-group",
            my_fake_users.iter().map(|name| rsx!(li {
                color: "rgb(220, 221, 222)",
                class: "list-group-item",
                UserStatus { name: name}
            }))
        }
    })
}

fn ChannelColumn(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            grid_area: "a",
            display: "grid",
            background: "rgb(32, 34, 37)"
        }
    })
}

fn UserColumn(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            grid_area: "b",
            display: "grid",
            background: "rgb(47, 49, 54)",
            Users {}
        }
    })
}

fn DisplayColumn(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            grid_area: "c",
            display: "grid",
            background: "rgb(54, 57, 63)",
            color: "rgb(220, 221, 222)",
            span {
                class: "h1",
                "Hello, many worlds!"
            }
        }
    })
}

fn UserInput(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            grid_area: "d",
            display: "grid",
            input {}
        }
    })
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            position: "absolute",
            top: "0",
            bottom: "0",
            left: "0",
            right: "0",
            display: "grid",
            grid_template_columns: "8ch 30ch 1fr",
            grid_template_rows: "1fr min-content",
            grid_template_areas: "\"a b c\" \"a b d\"",
            ChannelColumn {},
            UserColumn {},
            DisplayColumn {},
            UserInput {}
        }
    ))
}
