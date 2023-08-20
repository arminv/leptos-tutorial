use leptos::{ev::SubmitEvent, html::Input, *};

/// Shows progress toward a goal.
#[component]
fn ProgressBar(
    // All components take a reactive `Scope` as the first argument
    cx: Scope,
    // Marks this as an optional prop. It will default to the default
    // value of its type, i.e., 0.
    #[prop(default = 100)]
    /// The maximum value of the progress bar.
    max: u16,
    // Will run `.into()` on the value passed into the prop.
    #[prop(into)]
    // `Signal<T>` is a wrapper for several reactive types.
    // It can be helpful in component APIs like this, where we
    // might want to take any kind of reactive value
    /// How much progress should be displayed.
    progress: Signal<i32>,
) -> impl IntoView {
    view! { cx,
        <progress
            max={max}
            value={move || progress.get()}
        />
        <br/>
    }
}

#[component]
fn AppOne(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let double_count = move || count.get() * 2;

    view! { cx,
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            class:red = move || count.get() % 2 == 1
        >
            "Click me"
        </button>
        <br/>
        <ProgressBar max=50 progress=count/>
        <ProgressBar progress=count/>
        <ProgressBar max=50 progress=Signal::derive(cx, double_count)/>
    }
}

/// A list of counters, without the ability
/// to add or remove any.
#[component]
fn StaticList(
    cx: Scope,
    /// How many counters to include in this list.
    length: usize,
) -> impl IntoView {
    let counters = (1..=length).map(|idx| create_signal(cx, idx));

    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! { cx,
                <li>
                    <button
                        on:click=move |_| set_count.update(|n| *n += 1)
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect::<Vec<_>>();

    view! { cx,
        <ul>{counter_buttons}</ul>
    }
}

/// A list of counters that allows you to add or
/// remove counters.
#[component]
fn DynamicList(
    cx: Scope,
    /// The number of counters to begin with.
    initial_length: usize,
) -> impl IntoView {
    let mut next_counter_id = initial_length;

    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(cx, id + 1)))
        .collect::<Vec<_>>();

    let (counters, set_counters) = create_signal(cx, initial_counters);

    let add_counter = move |_| {
        let sig = create_signal(cx, next_counter_id + 1);
        set_counters.update(move |counters| counters.push((next_counter_id, sig)));
        // increment the ID so it's always unique
        next_counter_id += 1;
    };

    view! { cx,
        <div>
            <button on:click=add_counter>
                "Add Counter"
            </button>
            <ul>
                <For
                    each=move || counters.get()
                    key=|counter| counter.0
                    view=move |cx, (id, (count, set_count))| {
                        view! { cx,
                            <li>
                                <button
                                    on:click=move |_| set_count.update(|n| *n += 1)
                                >
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters.update(|counters| {
                                            counters.retain(|(counter_id, _)| counter_id != &id)
                                        });
                                    }
                                >
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn AppTwo(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Iteration"</h1>
        <h2>"Static List"</h2>
        <p>"Use this pattern if the list itself is static."</p>
        <StaticList length=5/>
        <h2>"Dynamic List"</h2>
        <p>"Use this pattern if the rows in your list will change."</p>
        <DynamicList initial_length=5/>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "Controlled".to_string());

    let (name_two, set_name_two) = create_signal(cx, "Uncontrolled".to_string());

    let input_element: NodeRef<Input> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element.get().expect("<input> to exist").value();

        set_name_two.set(value);
    };

    view! {cx,
    <input
    type="text"
    on:input=move |ev| {
        set_name.set(event_target_value(&ev));
    }
    prop:value=name.get()
    />
    <p>"Name is:" {name}</p>

    <form on:submit=on_submit>
    <input type="text"
    value=name_two.get()
    node_ref=input_element
    />
    <input type="submit" value="Submit"/>
    </form>
    <p>"Name Two is:" {name_two}</p>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
