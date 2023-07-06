use capybara_lemmy_client::{sensitive::Sensitive, CapyClient};
use gloo::storage::{LocalStorage, Storage};
use leptos::*;
use serde::{Deserialize, Serialize};

use crate::app::CurrentUser;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LoginInfo {
    pub jwt: Sensitive<String>,
    pub instance: String,
    pub username: String,
}

impl Settings {
    pub fn get_logins() -> Vec<LoginInfo> {
        LocalStorage::get("logins").ok().unwrap_or_default()
    }

    pub fn create_login(cx: Scope, login: LoginInfo) {
        LocalStorage::set("current_login", login.clone()).unwrap();
        let capy_client = use_context::<CapyClient>(cx).unwrap();
        capy_client.set_jwt(Some(login.jwt.clone()));
        let current_user = use_context::<CurrentUser>(cx).unwrap();
        current_user.0.set(Some(login.clone()));

        let mut logins = Self::get_logins();
        logins.push(login);
        LocalStorage::set("logins", logins).unwrap();
    }

    pub fn set_current_login(login: Option<LoginInfo>) {
        LocalStorage::set("current_login", login).unwrap();
    }

    pub fn current_login() -> Option<LoginInfo> {
        LocalStorage::get("current_login").ok()
    }
}
