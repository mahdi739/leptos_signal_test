use leptos::*;
use leptos_use::storage::{use_local_storage, JsonCodec};

fn main() {
  console_log::init_with_level(log::Level::Debug).unwrap_or_default();
  console_error_panic_hook::set_once();
  mount_to_body(App);
}
#[component]
fn App() -> impl IntoView {
  let (counters, set_counters, _) = use_local_storage::<Vec<RwSignal<i32>>, JsonCodec>("counters");
  view! {
    <button on:click=move |_| set_counters.update(|f| f.push(RwSignal::new(0)))>"ADD COUNTER"</button>
    <button on:click=move |_| counters.get().last_mut().unwrap().update(|a| *a += 1)>"Increment LAST ONE"</button>
    <div>
      <h1>map:</h1>
      <ul>
        {move || {
            counters
                .get()
                .into_iter()
                .map(|counter| {
                    view! {
                      <li>
                        <button on:click=move |_| counter.update(|a| *a += 1)>Increment: {counter}</button>
                      </li>
                    }
                })
                .collect_view()
        }}

      </ul>
    </div>
    <div>
      <h1>For:</h1>
      <ul>
        <For each=move || counters.get().into_iter().enumerate() key=move |counter| counter.0 let:counter>
          <li>
            <button on:click=move |_| { counter.1.update(|a| *a += 1) }>Increment: {move || format!("{:?}", counter.1.get())}</button>
          </li>
        </For>
      </ul>
    </div>
  }
}

