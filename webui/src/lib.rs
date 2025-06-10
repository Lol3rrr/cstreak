use leptos::prelude::*;

use futures_util::StreamExt;

use core::pin::pin;

#[component]
pub fn ProfileInput(
    start_profile: cstreak::Profile,
    set_profile: WriteSignal<cstreak::Profile>,
) -> impl IntoView  {

    let (rank, set_rank) = signal(start_profile.level);
    let (xp, set_xp) = signal(start_profile.xp);
    
    let submit = move |_| {
        let profile = cstreak::Profile {
            xp: xp(),
            level: rank(),
        };
        set_profile(profile);
    };

    let profile_style = stylers::style! {
        "profile_input",
        div {
            display: grid;
            grid-template-columns: 75px 75px;

            column-gap: 3px;
            row-gap: 2px;
        }

        div > button {
            grid-column: 1 / 3;

            border-radius: 5px;
        }

        input {
            width: 75px;

            padding: 2px;
            border-radius: 2px;

            border: none;
            color: #e0fbfc;
            background-color: #3d5a80;
        }
        input:focus {
            border-color: #e0fbfc;
        }

        button {
            width: 100%;
            margin: auto;

            border-top: none;
            border-left: none;

            color: #ee6c4d;
            background-color: #3d5a80;

            font-family: "Roboto Mono", monospace;
            font-weight: 600;
            font-style: normal;
        }
    };

    view! {
        class = profile_style,
        <div>
            <label> Rank: </label>
            <input type="text" on:input=move |ev| {
                let raw_value = event_target_value(&ev);
                set_rank.update(move |rank| {
                    match raw_value.parse() {
                        Ok(v) => { *rank = v; }
                        _ => {}
                    };
                });
            } prop:value=rank />

            <label> Level: </label>
            <input type="text" on:input=move |ev| {
                let raw_value = event_target_value(&ev);
                set_xp.update(move |xp| {
                    match raw_value.parse() {
                        Ok(v) => { *xp = v; }
                        _ => {}
                    };
                });
            } prop:value=xp />

            <button on:click=submit> Confirm </button>
        </div>
    }
}

pub fn localstorage<T>(storage: web_sys::Storage, name: &'static str, data: ReadSignal<T>, set_initial: impl FnOnce(T)) where T: Default + serde::Serialize + serde::de::DeserializeOwned + 'static, ReadSignal<T>: ToStream<T> {
    let start_value = storage.get(name).unwrap().map(|v| serde_json::from_str::<T>(&v).ok()).flatten().unwrap_or_default();
    set_initial(start_value);

    leptos::reactive::spawn_local(async move {
        let mut stream = pin!(data.to_stream());

        while let Some(v) = stream.next().await {
            let value = serde_json::to_string(&v).unwrap();
            storage.set(name, &value).unwrap();
        }
    });
}
