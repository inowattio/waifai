use crate::error::WFResult;

pub trait Hotspot {
    /// Create a hotspot for the current interface, it may or may not have a password, note that
    /// this method doesnt start the hotspot, that's handled by [crate](Hotspot::create).
    fn create(&self, ssid: String, password: Option<String>) -> WFResult<()>; // TODO: what happens if it creates over an already created one?
    /// Starts the previously created hotspot, this is not blocking.
    /// Doesn't fail if the hotspot is already created.
    fn start(&self) -> WFResult<()>;
    /// Stops created hotspot.
    fn stop(&self) -> WFResult<()>;
    // fn clients(&self) -> WFResult<Vec<String>>;
    // fn is_active(&self) -> WFResult<bool>;
}
