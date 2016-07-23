#[cfg(feature = "serde_feature")]
extern crate serde;
#[cfg(feature = "serde_feature")]
extern crate serde_json;
#[cfg(feature = "serde_feature")]
extern crate serde_xml;

#[cfg(feature = "serde_feature")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

#[cfg(not(feature = "serde_feature"))]
include!("lib.rs.in");
