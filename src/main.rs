mod services;

use crate::services::{get_lyrics, LyricsResponse};
use dioxus::prelude::*;
use reqwest::{Client, Error, Response};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Home {}

    }
}

#[component]
pub fn Home() -> Element {
    let mut artist = use_signal(|| String::from(""));
    let mut title = use_signal(|| String::from(""));
    let mut lyrics = use_signal(|| String::new());

    rsx! {
    nav {
        class:"flex-no-wrap relative flex w-full items-center justify-between bg-gray-500",
        div {
            class : "flex w-full flex-wrap items-center justify-between px-3 ",

                div {
                    class : "font-bold  \
                    dark:text-white dark:bg-gray-500 p-2 align-right size-14",
                     "Lyrics Finder"
                    }

                div {
                    class : "font-bold  \
                    dark:text-white dark:bg-gray-500 p-2 relative flex items-center",
                     "by GM"
                    }


            }
    }
    div {
        class :"flex flex-row flex-auto  justify-center w-full  ",
        div {
            class: "flex flex-col m-2",
            input{
                class : "p-2 m-4 border-2 border-gray-300 rounded-md dark:bg-gray-800",
                placeholder:"Artist",
                value: artist,
                onchange : move |e| artist.set(e.value())
            }
        }
        div {
            class: "flex flex-col items-center justify-center",
            input{
                class : "p-2 m-4 border-2 border-gray-300 rounded-md dark:bg-gray-800",
                placeholder:"Song title",
                value:title,
                onchange : move |e| title.set(e.value())

            }
        }

        div {
            class: "flex flex-col xs:flex-row items-center justify-center",
            button {
                class: "btn btn-primary p-2 border-2 border-gray-300 rounded-md xs:flex-row \
                hover:bg-gray-400 hover:text-black dark:bg-gray-800",
                onclick: move |_| async move  {
                    lyrics.set(get_lyrics(&*artist.read(), &*title.read()).await.
                        unwrap_or(LyricsResponse {lyrics:" Not found ...".to_string()}).lyrics)
                },
                "Search song "
            }

        hr {}
        }

        }

        div {
            class : "flex flex-col text-center  font-bold text-xl ",

           textarea {
            resize:"none",
            text_align:"center",
            text_justify:"center",
            disabled: true,
            background_color: "transparent",
            line_break: "anywhere",
            class: "whitespace-pre-line lines-2",
            text_wrap: true,
            rows :"30",
            "{lyrics}"
              }
        }
    }
}
