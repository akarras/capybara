use capybara_lemmy_client::{sensitive::Sensitive, CapyClient};
use gloo::storage::{LocalStorage, Storage};
use leptos::*;
use serde::{Deserialize, Serialize};

use crate::app::CurrentUser;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings;

impl Settings {
    pub fn set_login(cx: Scope, jwt: Option<Sensitive<String>>) {
        LocalStorage::set("current_jwt", jwt.clone()).unwrap();
        let capy_client = use_context::<CapyClient>(cx).unwrap();
        capy_client.set_jwt(jwt.clone());
        let current_user = use_context::<CurrentUser>(cx).unwrap();
        current_user.0.set(jwt);
    }

    pub fn current_login() -> Option<Sensitive<String>> {
        LocalStorage::get::<Sensitive<String>>("current_jwt").ok()
    }
}
