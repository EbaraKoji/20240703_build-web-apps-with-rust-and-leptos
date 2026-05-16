use leptos::{ev::MouseEvent, *};

#[component]
pub fn Parent() -> impl IntoView {
    let (counter, set_counter) = create_signal::<i16>(0);

    let increment_counter = move |_| set_counter.update(|c| *c += 1);
    let decrement_counter = move |_| set_counter.update(|c| *c -= 1);

    view! {
        <div style="border: 1px solid black; argin: 4px">
            <h3>"Parent Closure"</h3>
            <p>"Counter: "{counter}</p>
            <div>
                <button type="button" on:click=increment_counter>
                    "Parent +1"
                </button>
                <button type="button" on:click=decrement_counter>
                    "Parent -1"
                </button>
            </div>
            <Child counter on_increment=increment_counter on_decrement=decrement_counter />
        </div>
    }
}

#[component]
pub fn Child<Inc, Dec>(
    counter: ReadSignal<i16>,
    on_increment: Inc,
    on_decrement: Dec,
) -> impl IntoView
where
    Inc: Fn(MouseEvent) + 'static,
    Dec: Fn(MouseEvent) + 'static,
{
    view! {
        <div style="border: 1px solid black; argin: 4px">
            <h3>"Child Closure"</h3>
            <p>"Counter: "{counter}</p>
            <div>
                <button type="button" on:click=on_increment>
                    "Parent +1"
                </button>
                <button type="button" on:click=on_decrement>
                    "Parent -1"
                </button>
            </div>
        </div>
    }
}
