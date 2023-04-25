use yew::prelude::*;

pub struct Billing {
    link: ComponentLink<Self>,
    selected_plan: u8,
}

pub enum Msg {
    SelectPlan(u8),
}

impl Component for Billing {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            selected_plan: 2,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SelectPlan(plan) => {
                self.selected_plan = plan;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{ "Billing" }</h1>
                <div class="advertisement">
                    <div class="left-option">
                        <p>{ "$16" }</p>
                        <p>{ "/year" }</p>
                        <p>{ "1 time" }</p>
                        <input type="radio" name="plan" checked=self.selected_plan == 1 onclick=self.link.callback(|_| Msg::SelectPlan(1)) />
                    </div>
                    <div class="main-option">
                        <p><del>{ "6" }</del>{ "5" }</p>
                        <p>{ "3 months" }</p>
                        <input type="radio" name="plan" checked=self.selected_plan == 2 onclick=self.link.callback(|_| Msg::SelectPlan(2)) />
                    </div>
                    <div class="right-option">
                        <p>{ "$2" }</p>
                        <p>{ "monthly" }</p>
                        <p>{ "(reoccurring)" }</p>
                        <input type="radio" name="plan" checked=self.selected_plan == 3 onclick=self.link.callback(|_| Msg::SelectPlan(3)) />
                    </div>
                </div>
                <button onclick=self.link.callback(|_| Msg::PayWithGoogle)>{ "Pay with Google" }</button>
                <button onclick=self.link.callback(|_| Msg::PayWithApple)>{ "Pay with Apple" }</button>
                <button onclick=self.link.callback(|_| Msg::PayWithPayPal)>{ "Pay with PayPal" }</button>
                <button onclick=self.link.callback(|_| Msg::PayWithCashApp)>{ "Pay with Cash App" }</button>
                <button onclick=self.link.callback(|_| Msg::PayWithVenmo)>{ "Pay with Venmo" }</button>
            </>
        }
    }
}
