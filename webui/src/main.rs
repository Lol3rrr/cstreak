use leptos::prelude::*;
use stylers::style;
use leptos::prelude::GetUntracked as LGetUntracked;
use leptos::prelude::Set as LSet;

use webui::ProfileInput;

#[component]
fn App() -> impl IntoView {
    let (expected_score, set_expected_score) = signal(600);

    let (start_profile, set_start_profile) = signal(cstreak::Profile::default());
    let (current_profile, set_current_profile) = signal(cstreak::Profile::default());
    
    let (mission_xp, set_mission_xp) = signal(0);

    let storage = window().local_storage().unwrap().unwrap();

    webui::localstorage::<_, cstreak::Profile>(storage.clone(), "start_profile", start_profile, |initial_value| {
        set_start_profile.set(initial_value);
    });
    webui::localstorage::<_, cstreak::Profile>(storage.clone(), "current_profile", current_profile, |initial_value| {
        set_current_profile.set(initial_value);
    });
    webui::localstorage::<_, i64>(storage.clone(), "mission_xp", mission_xp, |initial_value| {
        set_mission_xp.set(initial_value);
    });

    let progress_value = move || {
        let start = start_profile();
        let current = current_profile();

        start.earned_xp(&current).0
    };
    let remaining_games = move || {
        let start = start_profile();
        let current = current_profile();

        let earned = start.earned_xp(&current);
        earned.expected_games(cstreak::Game::Deathmatch { score: expected_score() }, mission_xp())
    };

    let target = move || {
        let start = start_profile();
        start.target_profile(mission_xp())
    };

    let total_xp_target = move || {
        11167 + mission_xp()
    };

    // Color Theme (https://venngage.com/blog/blue-color-palettes/):
    // * 3d5a80
    // * 98c1d9
    // * e0fbfc
    // * ee6c4d
    // * 293241
    let progress_class = style! {
        "progress_class",
        progress {
            border-radius: 7px;
            width: 60%;
        }
        progress::-webkit-progress-bar {
            background-color: #3d5a80;
            border-radius: 7px;
        }
        progress::-webkit-progress-value {
            background-color: #e0fbfc;
            border-radius: 7px;
        }

        .container {
            display: grid;
            grid-template-columns: 1fr 3fr 1fr;

            background-color: #293241;
            color: #ee6c4d;

            font-family: "Roboto Mono", monospace;
            font-weight: 600;
            font-style: normal;
            
            height: 20vh;
            margin-left: 4vw;
            margin-right: 4vw;
        }

        .container > div {
            justify-self: center;
            width: 100%;
            margin: auto;
        }

        .progress {
            text-align: center;
        }

        .progress-text > p {
            margin: 2px;
        }

        hr {
            border-top: 1px solid #3d5a80;
            border-color: #3d5a80;
        }

        p, label {
            color: #ee6c4d;

            font-family: "Roboto Mono", monospace;
            font-weight: 600;
            font-style: normal;
        }
    };

    view! {
        class = progress_class,
        <div class="container">
            <div>
                <p>Start Profile</p>
                <ProfileInput start_profile=start_profile.get_untracked() set_profile=set_start_profile />
            </div>

            <div class="progress">
                <div>
                    <p> {progress_value} "/" {total_xp_target} - { move || format!("{:03.02}%", progress_value() as f32 / (total_xp_target() as f32) * 100.0) } </p>
                </div>
                <progress max={total_xp_target} value = progress_value />
                <div class="progress-text">
                    <p> Missing { move || total_xp_target() - progress_value() } </p>
                    <p>Target-Level: { move || target().level } </p>
                    <p>Target-XP: { move || target().xp } </p>
                </div>
            </div>

            <div>
                <p>Current Profile</p>
                <ProfileInput start_profile=current_profile.get_untracked() set_profile=set_current_profile />
            </div>
        </div>

        <div>
            <label>Mission XP:</label>
            <input type="text" on:input=move |ev| {
                let raw_value = event_target_value(&ev);
                set_mission_xp.update(move |xp| {
                    match raw_value.parse() {
                        Ok(v) => {
                            *xp = v;
                        }
                        _ => {}
                    };
                });
            } prop:value=mission_xp />
        </div>

        <hr/>

        <div>
            <p> "Deathmatch" </p>
            <label> "Expected Score:" </label>
            <input type="text" on:input=move |ev| {
                let raw_value = event_target_value(&ev);
                set_expected_score.update(move |xp| {
                    match raw_value.parse() {
                        Ok(v) if v > 0 => { *xp = v; }
                        _ => {}
                    };
                });
            } prop:value=expected_score />

            <p> "Remaining Games: " {remaining_games} </p>
            <p> "Duration: " {move || { format!("{:01}h {:02}m", remaining_games() * 10 / 60, remaining_games() * 10 % 60) }} </p>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
