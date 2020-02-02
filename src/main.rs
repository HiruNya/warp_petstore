mod pet;
mod store;
mod user;
mod util;

use warp::{document::{describe, to_openapi}, Filter};

#[tokio::main]
async fn main() {
	let routes = pet::pet().or(store::store()).or(user::user());

	let documentation = to_openapi(describe(&routes));

	#[cfg(not(feature = "serve"))]
	println!("{}", serde_json::to_string_pretty(&documentation).unwrap());

	#[cfg(feature = "serve")]
	{
		use warp::{filters::{fs::file, path::path}, reply::json};

		// Lets serve the specification as well at /openapi.json
		// And the documentation will be at /docs
		let routes = routes.or(path("openapi.json").map(move || json(&documentation)))
			.or(path("docs").and(file("./public/index.html")));
		warp::serve(routes).run(([127, 0, 0, 1], 8080)).await
	}
}
