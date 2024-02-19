use std::time::Duration;

pub trait TickInterpolator<T> {
    fn tick(&mut self, dt: Duration);
    fn set_target(&mut self, target: T);
    fn get(&self) -> T;
}
