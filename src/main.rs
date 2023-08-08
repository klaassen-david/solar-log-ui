use std::cell::OnceCell;

use leptos::*;
use plotly::{layout::Axis, Layout, Plot};

use crate::state::{GlobalState, Trace};

mod log;
mod state;

const PLOT_ID: &str = "plot_id";

fn main() {
    mount_to_body(|cx| {
        view! { cx, <App /> }
    })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (state, set_state) = create_signal(cx, GlobalState::default());
    let (log, set_log) = create_signal(
        cx,
        OnceCell::from(log::Log::parse(log::file::CONTENT.to_string()).unwrap()),
    );
    log!(
        "read {:?} log entries",
        log.get_untracked().get().unwrap().entries[0].len()
    );

    let render = create_action(cx, move |_: &()| {
        let mut plot = Plot::new();
        for trace in state.get().traces.into_iter() {
            // log!("{:?}", trace);
            let trace = trace.1.render(log.get());
            plot.add_trace(trace);
        }
        let layout = Layout::new()
            .title(plotly::common::Title::new("best chart"))
            .x_axis(Axis::default().type_(plotly::layout::AxisType::Date));
        plot.set_layout(layout);

        async move {
            plotly::bindings::new_plot(PLOT_ID, &plot).await;
        }
    });
    let css = ".dropdown {
    display: inline-block;
    position: relative;
}";
    let mut next_controller_id = 0usize;

    let add_controller = move |_| {
        let idx = create_rw_signal(
            cx,
            if let Some((_, Trace { idx, .. })) = state.get().traces.last() {
                idx.get_untracked()
            } else {
                0usize
            },
        );
        let trace = Trace { idx };
        set_state.update(|s| s.traces.push((next_controller_id, trace)));
        create_effect(cx, move |_| {
            log! {"id: {:?}, idx: {:?}", next_controller_id.clone(), idx.get()};
        });
        next_controller_id += 1;
        render.dispatch(());
    };

    view! { cx,
        <style>{move || css}</style>
        <script src="https://cdn.plot.ly/plotly-2.14.0.min.js"></script>
        <button
            on:click=add_controller
        > "Add Trace"
        </button>

        <div id=PLOT_ID>
        </div>
    <ul>
    <For
        each=move || {state.get().traces}
        key=move |k| k.0
        view=move |cx, (_id, t)| {view! {cx, <li>
        <select name="idx"
            on:change=move |ev| {t.idx.set(event_target_value(&ev).parse().unwrap());
            ev.prevent_default();
    render.dispatch(())}
        >
            <For
            each=move || {(0usize..log.get().get().unwrap().keys.len()).into_iter()}
            key=move |k| k.clone()
            view=move|cx, idx| {
                    // log!{"{}: {}", idx, log.get().get().unwrap().get_legend(idx)};
                    view! {cx, <option value=idx selected=move || {t.idx.get() == idx}>{log.get().get().unwrap().get_legend(idx)}</option>}
                }
            />
        </select>
        </li>}
        }
    />
    </ul>

    }
}
