#[cfg(not(feature = "blocking"))]
mod async_tests;

#[cfg(feature = "blocking")]
mod blocking_tests;
