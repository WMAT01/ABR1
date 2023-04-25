use yew::prelude::*;

//pub struct SignInPage;
pub struct SignInPage {
    link: ComponentLink<Self>,
    on_google_signin: Callback<()>,
}

pub enum Msg {
    GoogleSignIn,
}

impl SignInPage {
    fn google_sign_in(&self) {
        let window = web_sys::window().unwrap();
        let gapi = window.get("gapi").unwrap();
        let auth2 = js_sys::Reflect::get(&gapi, &"auth2").unwrap();
        let instance = js_sys::Reflect::get(&auth2, &"getAuthInstance").unwrap();
        let user = js_sys::Reflect::apply(&instance, &auth2, &js_sys::Array::new()).unwrap();

        let is_signed_in = js_sys::Reflect::get(&user, &"isSignedIn").unwrap();
        let is_signed_in_bool = js_sys::Reflect::apply(&is_signed_in, &user, &js_sys::Array::new())
            .unwrap()
            .as_bool()
            .unwrap();

        if !is_signed_in_bool {
            let sign_in = js_sys::Reflect::get(&user, &"signIn").unwrap();
            let _ = js_sys::Reflect::apply(&sign_in, &user, &js_sys::Array::new()).unwrap();
        }
    }
}


impl Component for SignInPage {
    //~ type Message = ();
    //~ type Properties = ();

    //~ fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        //~ Self
    //~ }

    //~ fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        //~ false
    //~ }
type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let on_google_signin = link.callback(|_| Msg::GoogleSignIn);
        Self {
            link,
            on_google_signin,
        }
    }

    //~ fn update(&mut self, msg: Self::Message) -> ShouldRender {
        //~ match msg {
            //~ Msg::GoogleSignIn => {
                //~ // Handle Google Sign-In
            //~ }
        //~ }
        //~ false
    //~ }
fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
        Msg::GoogleSignIn => {
            self.google_sign_in();
        }
    }
    false
}



    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    //~ fn view(&self) -> Html {
        //~ html! {
            //~ <div class="sign-in-container">
                //~ <h1 class="sign-in-title">{"Sign In"}</h1>
                //~ <div class="sign-in-options">
                    //~ <button class="sign-in-btn" id="google-signin">{"Sign in with Google"}</button>
                    //~ <button class="sign-in-btn" id="microsoft-signin">{"Sign in with Microsoft"}</button>
                    //~ <button class="sign-in-btn" id="twitter-signin">{"Sign in with Twitter"}</button>
                    //~ <button class="sign-in-btn" id="reddit-signin">{"Sign in with Reddit"}</button>
                    //~ <button class="sign-in-btn" id="facebook-signin">{"Sign in with Facebook"}</button>
                    //~ <button class="sign-in-btn" id="amazon-signin">{"Sign in with Amazon"}</button>
                    //~ <button class="sign-in-btn" id="apple-signin">{"Sign in with Apple"}</button>
                //~ </div>
            //~ </div>
        //~ }
    //~ }
    fn view(&self) -> Html {
        html! {
            <div class="container">
                <div class="logo">
                    // Your logo here
                </div>
                <h1 class="title">{ "ABR" }</h1>
                <p class="tagline">{ "Empowering Insights, Enhancing Expertise, Edging-out the Competition" }</p>
                <div class="sign-in-buttons">
                    <button onclick=self.link.callback(|_| Msg::GoogleSignIn)>
                        { "Sign in with Google" }
                    </button>
                    // Add more sign-in buttons for other providers here
                    <button class="sign-in-btn" id="microsoft-signin">{"Sign in with Microsoft"}</button>
                    <button class="sign-in-btn" id="twitter-signin">{"Sign in with Twitter"}</button>
                    <button class="sign-in-btn" id="reddit-signin">{"Sign in with Reddit"}</button>
                    <button class="sign-in-btn" id="facebook-signin">{"Sign in with Facebook"}</button>
                    <button class="sign-in-btn" id="amazon-signin">{"Sign in with Amazon"}</button>
                    <button class="sign-in-btn" id="apple-signin">{"Sign in with Apple"}</button>
                </div>
            </div>
        }
    }
}
