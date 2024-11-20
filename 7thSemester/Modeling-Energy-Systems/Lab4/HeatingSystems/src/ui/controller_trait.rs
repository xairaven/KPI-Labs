pub trait MenuController {
    fn view(&self);
    fn name(&self) -> String;
}
