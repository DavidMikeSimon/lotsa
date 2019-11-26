#[derive(Clone)]
pub struct DirtinessTracker {}

impl DirtinessTracker {
  pub fn new() -> DirtinessTracker { DirtinessTracker {} }
}

impl Default for DirtinessTracker {
  fn default() -> DirtinessTracker { DirtinessTracker::new() }
}
