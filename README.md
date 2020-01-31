This is a preview of using warp server to generate an OpenApi document.
This aims to mimic the [Pet Store API](https://petstore.swagger.io/)
using a [special fork of warp](https://github.com/HiruNya/warp).

[View the documentation generated here!](https://hiru.dev/demo/warp-petstore/)

[View the OpenAPI file generated here.](https://hiru.dev/demo/warp-petstore/openapi.json)

## This Crate

This binary does two things:
- Produce an OpenAPI JSON file (default) describing a warp server.
- Actually run a warp server (`cargo run --feature serve`)

## Possible Macros

I've written down some possible macros that would make documentation much easier to write.

### Struct Definitions

Currently the method of defining structs is tedious and prone to human error.
e.g.
```rust
use warp::document::{DocumentedType, integer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: u32,
    user_name: String,
    age: u8,
}
impl DocumentedType {
    fn document() -> DocumentedType {
        let mut hashmap = HashMap::with_capacity(3);
        map.insert("id", integer());
        // Needs to manually switch to camel case
        map.insert("userName", string().example("Hiru")); // Examples can be added
        map.insert("age", integer().description("The age of the *account*, NOT the user."));
        map.into()
    }
}
```

#### With Macros
```rust
use warp::document::ToDocumentedType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, ToDocumentedType)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: u32,
    #[example("Hiru")]
    user_name: String,
    /// The age of the *account*, NOT the user.
    age: u8,
}
```
This relies on a few assumptions:
- Procedural Derive Macros can see attributes that don't belong to it. e.g. `#[serde(...)]`.
[This implies that it should work.](https://doc.rust-lang.org/reference/procedural-macros.html#derive-macro-helper-attributes)
- Documentation comments desugar to #[doc(...)] attributes.

### Function Definitions

Currently we chain filters like this:
```rust
use warp::{any, document::{document, description, response}, Filter, Rejection, reply::Reply};

fn my_custom_filter() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    any()
        .and(document(description("This is my custom filter")))
        .and(document(response(200, None)))
        .map(|| "Success")
}
```
Using `warp::document::document` unfortunately only creates `Filter` that implement `Clone` but not `Copy`.

If we use `warp::document::exact`, we can avoid this by doing
```rust
use warp::{any, document::{self, response}, Filter, Rejection};

fn my_custom_filter() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    let filter = any().map(|| "Success");
    document::exact(filter, |route| {
        route.description("This is my custom filter");
        route.response(response(200, None))
    })
}
```
This function (theoretically) should be able to implement `Copy` because the filter only holds a function pointer
(please feel free to correct me).
However it introduces more boilerplate and somewhat ugly code.

#### With Macros
With a macro we could possibly do something like this:
```rust
use warp::{any, Filter, Rejection, reply::Reply};

#[warp_filter]
/// This is my custom filter
#[response(status = 200)]
fn my_custom_filter() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    any().map(|| "Success")
}
```
Which would turn into something like
```rust
use warp::{any, Filter, Rejection, reply::Reply};

fn my_custom_filter() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    let filter = {
        any().map(|| "Success")
    };
    warp::document::exact(filter, |route| {
        route.description("This is my custom filter");
        route.response(warp::response(200, None))
    })
}
```