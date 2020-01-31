mod pet;
mod store;
mod user;
mod util;

use warp::Filter;

#[tokio::main]
async fn main() {
	let routes = pet::pet().or(store::store()).or(user::user());

	#[cfg(not(feature = "serve"))]
	{
		use warp::document::{describe, to_openapi};
		let documentation = to_openapi(describe(routes));
		println!("{}", serde_json::to_string_pretty(&documentation).unwrap());
	}

	#[cfg(feature = "serve")]
	warp::serve(routes).run(([127, 0, 0, 1], 8080)).await
}
