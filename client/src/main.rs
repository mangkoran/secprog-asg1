use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/create")]
    Create,
    #[at("/login")]
    Login,
}

#[function_component]
fn Foo() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <>
            <div>
                <button {onclick}>{ "+1" }</button>
                <p>{ *counter }</p>
            </div>
        </>
    }
}

#[function_component]
fn Create() -> Html {
    html! {
        <>
            <h2> { "Create User" } </h2>
            <div>
                <form>
                    <input type="text" id="username" name="username"/><br/>
                    <input type="password" id="password" name="password"/><br/>
                </form>
            </div>
        </>
    }
}

#[function_component]
fn Login() -> Html {
    html! {
        <>
            <h2> { "Login" } </h2>
            <div>
                <form>
                    <input type="text" id="username" name="username"/><br/>
                    <input type="password" id="password" name="password"/><br/>
                </form>
            </div>
        </>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Foo /> },
        Route::Create => html! { <Create /> },
        Route::Login => html! { <Login /> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
