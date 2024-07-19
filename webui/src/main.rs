use leptos::*;
use stylers::style;

use webui::ProfileInput;

#[component]
fn App() -> impl IntoView {
    let (expected_score, set_expected_score) = create_signal(600);

    let (start_profile, set_start_profile) = create_signal(cstreak::Profile {
        xp: 0,
        level: 1,
    });

    let (current_profile, set_current_profile) = create_signal(cstreak::Profile {
        xp: 0,
        level: 1,
    });

    let (stored_start_profile, write_start_profile, delete_start_profile) = leptos_use::storage::use_local_storage::<cstreak::Profile, leptos_use::utils::JsonCodec>("start_profile");
    let (stored_current_profile, write_current_profile, delete_current_profile) = leptos_use::storage::use_local_storage::<cstreak::Profile, leptos_use::utils::JsonCodec>("current_profile");

    let progress_value = move || {
        let start = start_profile();
        let current = current_profile();

        write_start_profile(start.clone());
        write_current_profile(current.clone());

        start.earned_xp(&current).0
    };
    let remaining_games = move || {
        let start = start_profile();
        let current = current_profile();

        let earned = start.earned_xp(&current);
        earned.expected_games(cstreak::Game::Deathmatch { score: expected_score() })
    };

    set_start_profile.set_untracked(stored_start_profile.get_untracked());
    set_current_profile.set_untracked(stored_current_profile.get_untracked());

    // Color Theme (https://venngage.com/blog/blue-color-palettes/):
    // * 3d5a80
    // * 98c1d9
    // * e0fbfc
    // * ee6c4d
    // * 293241
    let progress_class = style! { 
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
                <progress max={11167} value = progress_value />
                <div class="progress-text">
                    <p> {progress_value} "/" {11167}  </p>
                    <p> { move || format!("{:03.02}%", progress_value() as f32 / 11167.0 * 100.0) } </p>
                </div>
            </div>

            <div>
                <p>Current Profile</p>
                <ProfileInput start_profile=current_profile.get_untracked() set_profile=set_current_profile />
            </div>
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
