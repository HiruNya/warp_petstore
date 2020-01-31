use warp::{Filter, Rejection};
use warp::document::{explicit, parameter, RouteDocumentation};
use std::any::TypeId;

/// Since the `warp::filters::path:::param` filter doesn't allow us to name the parameter
/// we'll have to make own version.
/// By default, `warp::filters::path::param` calls its parameter "Param1", "Param2", etc.
pub fn param<T>(name: &'static str, description: &'static str) -> impl Filter<Extract = (T,), Error = Rejection> + Copy
where
    T: std::marker::Send + std::str::FromStr + 'static,
{
    let filter = warp::filters::path::param::<T>();
    // `explicit` returns a filter that implements Copy as long as the function implements Copy.
    // This is unlike `document::document` which always only implements Clone.
    explicit(filter, move |route: &mut RouteDocumentation| {
        // After we call param, we take the last added parameter and change its name as desired.
        // TypeId implements Into<DocumentedType> by checking the type at runtime.
        route.parameter(parameter(name, TypeId::of::<T>()).description(description));
    })
}
