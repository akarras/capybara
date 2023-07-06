use leptos::*;

#[component]
pub fn NumberVis(cx: Scope, value: i64) -> impl IntoView {
    let val = value as f64;
    let (float_value, unit) = if value > 1_000_000 {
        (val / 1_000_000.0, "m")
    } else if value > 1_000 {
        (val / 1_000.0, "k")
    } else {
        (val, "")
    };
    format!("{float_value:.1}{unit}").into_view(cx)
}
