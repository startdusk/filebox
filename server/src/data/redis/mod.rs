pub mod ip_allow;

pub use ip_allow::*;

use crate::api::IpInfo;

impl TryFrom<String> for IpInfo {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&value)
    }
}
