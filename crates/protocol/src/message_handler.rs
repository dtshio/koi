use std::future::Future;

#[allow(unused_variables, unused_mut)]
pub trait ClientMessageHandler<T> {
    fn handle_event(&self, mut context: T, message: String) -> impl Future<Output = ()> + Send { async {} }
}

#[allow(unused_variables, unused_mut)]
pub trait ServerMessageHandler<T> {
    fn handle_connection(&self, mut context: T, address: String) -> impl Future<Output = ()> + Send { async {} }
}
