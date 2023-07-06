use capybara_lemmy_client::post::{ListingType, SortType};
use leptos::*;
use leptos_icons::*;

fn sort_to_text(sort_type: Option<SortType>) -> &'static str {
    match sort_type {
        None => "None",
        Some(SortType::Active) => "Active",
        Some(SortType::Hot) => "Hot",
        Some(SortType::New) => "New",
        Some(SortType::Old) => "Old",
        Some(SortType::TopDay) => "Top Day",
        Some(SortType::TopWeek) => "Top Week",
        Some(SortType::TopMonth) => "Top Month",
        Some(SortType::TopYear) => "Top Year",
        Some(SortType::TopAll) => "Top All",
        Some(SortType::MostComments) => "Most Comments",
        Some(SortType::NewComments) => "New Comments",
        Some(SortType::TopHour) => "Top Hour",
        Some(SortType::TopSixHour) => "Top 6 Hour",
        Some(SortType::TopTwelveHour) => "Top 12 Hour",
    }
}

#[component]
pub fn SortMenu(
    cx: Scope,
    sort: ReadSignal<Option<SortType>>,
    set_sort: WriteSignal<Option<SortType>>,
) -> impl IntoView {
    let sort_menu_hidden = create_rw_signal(cx, true);
    view! { cx,
        <div>
            <button
                class="p-1 bg-neutral-800 hover:bg-neutral-500 border-gray-300 border-b-3 flex flex-row align-bottom"
                on:click=move |_| {
                    sort_menu_hidden.update(|u| *u = !*u);
                }
            >
                <Icon icon=MaybeSignal::Static(BiIcon::BiSortDownRegular.into())/>
                {move || sort_to_text(sort())}
            </button>
            <div class="flex flex-col absolute z-30" class:hidden=sort_menu_hidden>
                <PostSort value=None set_sort sort_menu_hidden/>
                <PostSort value=Some(SortType::Active) set_sort sort_menu_hidden/>
                <PostSort value=Some(SortType::Hot) set_sort sort_menu_hidden/>
                <PostSort value=Some(SortType::New) set_sort sort_menu_hidden/>
                <PostSort value=Some(SortType::Old) set_sort sort_menu_hidden/>
                <PostSort value=Some(SortType::TopDay) set_sort sort_menu_hidden/>
                <PostSort value=Some(SortType::TopWeek) set_sort sort_menu_hidden/>
                <PostSort value=Some(SortType::TopMonth) set_sort sort_menu_hidden/>
                <PostSort value=Some(SortType::TopYear) set_sort sort_menu_hidden/>
                <PostSort value=Some(SortType::TopAll) set_sort sort_menu_hidden/>
                <PostSort value=Some(SortType::MostComments) set_sort sort_menu_hidden/>
                <PostSort value=Some(SortType::NewComments) set_sort sort_menu_hidden/>
                <PostSort value=Some(SortType::TopHour) set_sort sort_menu_hidden/>
                <PostSort value=Some(SortType::TopSixHour) set_sort sort_menu_hidden/>
                <PostSort value=Some(SortType::TopTwelveHour) set_sort sort_menu_hidden/>
            </div>
        </div>
    }
}

#[component]
fn PostSort(
    cx: Scope,
    value: Option<SortType>,
    set_sort: WriteSignal<Option<SortType>>,
    sort_menu_hidden: RwSignal<bool>,
) -> impl IntoView {
    view! { cx,
        <button
            class="bg-neutral-800 hover:bg-neutral-600"
            on:click=move |_| {
                set_sort(value);
                sort_menu_hidden.set(true);
            }
        >
            {sort_to_text(value)}
        </button>
    }
}

#[component]
pub fn TypeMenu(
    cx: Scope,
    type_: ReadSignal<Option<ListingType>>,
    set_type: WriteSignal<Option<ListingType>>,
) -> impl IntoView {
    let options = [
        (Some(ListingType::All), "All"),
        (Some(ListingType::Local), "Local"),
        (Some(ListingType::Subscribed), "Subscribed"),
    ];
    view! { cx,
        <div class="flex-row">
            {options
                .into_iter()
                .map(|(listing_type, name)| {
                    view! { cx,
                        <button
                            class:underline=move || type_() == listing_type
                            class="bg-neutral-800 hover:bg-neutral-500 border-gray-300 border-b-1 p-1 align-bottom"
                            on:click=move |_| {
                                set_type.set(listing_type);
                            }
                        >
                            {name}
                        </button>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
