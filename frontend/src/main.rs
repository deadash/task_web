#![feature(specialization)]

mod components;
mod services;

use crate::components::app::App;

pub fn main() {
    yew::Renderer::<App>::new().render();
}