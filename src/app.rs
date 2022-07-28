use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{Home, ProductDetail};
use crate::route::Route;
use crate::types::{CartProduct, Product};
use crate::components::Navbar;

struct State {
    cart_products: Vec<CartProduct>,
}

pub enum Msg {
    AddToCart(Product),
}

pub struct App {
    state: State,
    link: ComponentLink<Self>
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let cart_products: Vec<CartProduct> = vec![];
         
        Self {
            state: State { cart_products },
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddToCart(product) => {
                let cart_product = self
                .state
                .cart_products
                .iter_mut()
                .find(|cp: &&mut CartProduct| cp.product.id == product.id);
    
                if let Some(cp) = cart_product {
                    cp.quantity += 1;
                }
                else {
                    self.state.cart_products.push(CartProduct {
                        product: product.clone(),
                        quantity: 1
                    });
                }
                true
            }
        }
    }

    fn change(&mut self, _:Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let handler_add_to_cart = self
        .link
        .callback(|product: Product| Msg::AddToCart(product));
        let cart_products = self.state.cart_products.clone();

        let render = Router::render(move |switch: Route| match switch {
            Route::ProductDetail(id) => html! {
                <ProductDetail id=id on_add_to_cart=handler_add_to_cart.clone()/>
            },
            Route::HomePage => html! {
                <Home cart_products=cart_products.clone() on_add_to_cart=handler_add_to_cart.clone()/>
            }
        });

        html! {
            <>
            <Navbar cart_products=self.state.cart_products.clone() />
            <Router<Route, ()> render=render/>
            </>
        }
    }
}