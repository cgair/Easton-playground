// <https://stackoverflow.com/questions/48027839/how-to-box-a-trait-that-has-associated-types>
use yew::Component;

trait ComponentWrapper {
    fn create(ctx: &yew::Context<Self>) -> Self {
        todo!()
    }

}

impl<T: Component> ComponentWrapper for Option<T> {}

struct Item1;
impl Component for Item1 {
    type Message = String;
    type Properties = ();
    fn create(ctx: &yew::Context<Self>) -> Self {
        todo!()
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        todo!()
    }
}

struct Item2;
impl Component for Item2 {
    type Message = String;
    type Properties = ();
    fn create(ctx: &yew::Context<Self>) -> Self {
        todo!()
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        todo!()
    }
}

fn main() {
    let mut v: Vec<Box<dyn ComponentWrapper>> = vec![];
    v.push(Box::new(Some(Item1)));
    v.push(Box::new(Some(Item2)));
}
