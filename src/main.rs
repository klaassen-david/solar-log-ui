use leptos::{server_fn::server, *};
use leptos_chart::{ScatterChart, Series};
use log::Log;

mod log;

fn main() {
    let log = log::Log::parse(log::file::CONTENT.to_string()).unwrap();
    log!("{}", log.entries.len());
    mount_to_body(|cx| {
        view! { cx, <App /> }
    })
}

#[server(ReadLog, "/log")]
pub async fn read_log() -> Result<Log, ServerFnError> {
    log::Log::parse_file("/home/dk/code/kostalui/log.txt".to_string())
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    log!("parsing log");
    let log = log::Log::parse(log::file::CONTENT.to_string()).unwrap();
    log!("{}", log.entries.len());
    let chart = leptos_chart::Cartesian::new(
        Series::from(log.timestamps),
        Series::from(
            log.entries[0]
                .iter()
                .map(|i| i.unwrap_or(0.0))
                .collect::<Vec<_>>(),
        ),
    )
    .set_view(820, 620, 3, 100, 100, 20);
    log!("created chart");

    view! { cx,
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        > "Click me:" {move || count()}
        </button>

        <div class="mx-auto p-8">
            <h1>"Scatter chart example"</h1>
            <ScatterChart chart=chart />
        </div>

    }
}
