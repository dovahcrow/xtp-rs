#[allow(unused_variables)]
pub trait TraderSpi {
    fn on_disconnect(&self, reason: i32) {}
}
