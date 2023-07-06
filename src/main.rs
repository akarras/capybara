mod app;
pub mod community;
pub mod community_list;
pub mod components;
pub mod login;
pub mod settings;

use app::*;
use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <App/>
        }
    })
}
