#[cfg(feature = "user-msgs")]
pub mod common {
    include!(concat!(env!("OUT_DIR"), "/common.rs"));
}

#[cfg(any(feature = "gc-common", feature = "user-msgs", feature = "gc-client"))]
pub mod gcsdk {
    include!(concat!(env!("OUT_DIR"), "/gcsdk.rs"));
}

pub mod deadlock {
    include!(concat!(env!("OUT_DIR"), "/deadlock.rs"));
}
