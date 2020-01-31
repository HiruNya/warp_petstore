This is a preview of using warp server to generate an OpenApi document.
This aims to mimic the [Pet Store API](https://petstore.swagger.io/)
using a [special fork of warp](https://github.com/HiruNya/warp).

Throughout the code, I have left doc comments on what parts of the api
I feel could be simplified using procedural macros to remove the boilerplate.

## Possible Macros

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

####
With Macros
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
