mod budget;
mod demo;
mod export;
mod report;
mod serve;

pub use budget::set as budget_set;
pub use budget::show as budget_show;
pub use demo::run as demo;
pub use export::run as export;
pub use report::run as report;
pub use serve::run as dashboard;
pub use serve::serve_only;

use carbotrack_core::{CarboTrack, CarboTrackConfig};

pub(crate) fn demo_tracker() -> CarboTrack {
    CarboTrack::new(CarboTrackConfig {
        cloud: "aws".into(),
        region: "us-east-1".into(),
        budget_gco2e_per_day: Some(5.0),
        ..Default::default()
    })
}
