use crate::error::WFResult;

pub trait Hotspot {
    /// Create a hotspot for the current interface, it may or may not have a password, note that
    /// this method doesn't start the hotspot, that's handled by [crate](Hotspot::create).
    /// Note that this creates a new connection every time the function is used (will solve).
    // TODO: Fix above.
    fn create(&self, ssid: &str, password: Option<&str>) -> WFResult<()>;
    /// Starts the previously created hotspot, this is not blocking.
    /// Doesn't fail if the hotspot is already created.

    fn start(&self) -> WFResult<()>;
    /// Stops created hotspot.
    fn stop(&self) -> WFResult<()>;
    // fn clients(&self) -> WFResult<Vec<String>>;

    /// Returns if the current interface is running a hotspot.
    fn is_active(&self) -> WFResult<bool>;
}
