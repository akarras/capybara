use chrono::{NaiveDateTime, Utc};
use leptos::*;

#[component]
pub fn RelativeTime(cx: Scope, time: NaiveDateTime) -> impl IntoView {
    let duration = Utc::now().naive_utc() - time;
    let weeks = duration.num_weeks();
    let days = duration.num_days();
    let hours = duration.num_hours();
    let minutes = duration.num_minutes();
    let (num, d) = if weeks >= 1 {
        (weeks, "w")
    } else if days >= 1 {
        (days, "d")
    } else if hours >= 1 {
        (hours, "h")
    } else {
        (minutes, "m")
    };
    view! {cx, <span>{num}{d}</span>}
}
