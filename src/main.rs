use leptos::*;

mod log;
const PLOT_ID: &str = "plotid";

fn main() {
    mount_to_body(|cx| view! { cx, <App /> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let log = log::Log::parse_file("log.txt".to_string()).unwrap();
    dbg!(log);

    let data = create_resource(
        cx,
        || (),
        |_| async move {
            let log = log::Log::parse_file("log.txt".to_string()).unwrap();
            let mut plot = plotly::Plot::new();
            let trace = plotly::Scatter::new(log.timestamps.clone(), *log.entries[3].clone());
            plot.add_trace(trace);
            plotly::bindings::new_plot(PLOT_ID, &plot).await;
        },
    );

    view! { cx,
        <script src="https://cdn.plot.ly/plotly-2.14.0.min.js"></script>
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        > "Click me:" {move || count()}
        </button>

        {move || match data.read(cx) {
                None => view! {cx, <p>"Loading..."</p> }.into_view(cx),
                Some(_) => view! {cx, <p>"Loaded"</p> }.into_view(cx),
            }}
        <div
            id=PLOT_ID
        ></div>

    }
}
