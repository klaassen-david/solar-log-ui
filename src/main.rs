use leptos::{server_fn::server, *};
use log::Log;
use plotly::{Layout, Plot, Scatter};

mod log;

const PLOT_ID: &str = "plot_id";

fn main() {
    let log = log::Log::parse(log::file::CONTENT.to_string()).unwrap();
    log!("{:?}", log.entries[0].len());
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

    let log = log::Log::parse(log::file::CONTENT.to_string()).unwrap();
    let mut plot = Plot::new();
    let trace = Scatter::new(
        log.timestamps,
        log.entries[0]
            .iter()
            .map(|i| i.unwrap_or(0.0))
            .collect::<Vec<_>>(),
    );
    plot.add_trace(trace);
    let layout = Layout::new().title(plotly::common::Title::new("best chart"));
    plot.set_layout(layout);
    let render = create_action(cx, move |_input: &()| {
        let plot = plot.to_owned();
        log!("rendering");
        async move {
            log!("plotly start");
            plotly::bindings::new_plot(PLOT_ID, &plot).await;
            log!("plotly end");
        }
        // async move { plotly::bindings::new_plot(&input.0, input.1).await }
    });
    log!("created chart prerequisites");

    view! { cx,
        <script src="https://cdn.plot.ly/plotly-2.14.0.min.js"></script>
        <button
            on:click=move |_| {
            set_count.update(|n| *n += 1);
            render.dispatch(())
            }
        > "Click me:" {move || count()}
        </button>

        <div id=PLOT_ID>
        </div>

    }
}
