use warp::{body, Filter, filters::{header::header, query::query, path::path}, Rejection, reply::{self, Reply}};
use warp::document::{self, array, body, description, DocumentedType, integer, response, RouteDocumentation, string};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::util::param;

pub fn pet() -> impl Filter<Extract=impl Reply, Error=Rejection> + Clone {
	warp::any().and(path("pet"))
		.and(
			get_pet()
				.or(delete_pet())
				.or(pet_post())
				.or(pet_post())
				.or(pet_put())
				.or(pet_status())
		)
}

fn pet_id() -> impl Filter<Extract=(usize,), Error=Rejection> + Clone {
	param("petId", "The id of the pet")
		// If we can't find the pet, we return a 404
		.and(document::document(response(404, None).description("The pet could not be found")))
}

fn get_pet() -> impl Filter<Extract=impl Reply, Error=Rejection> + Clone {
	pet_id()
		.and(warp::get())
		.and(document::document(description("Returns a single pet object")))
		.and(document::document(response(200, body(pet_struct()).mime("application/json")).description("Successful Operation!")))
		.map(|id| reply::json(&Pet { id, ..Pet::default() }))
}

fn delete_pet() -> impl Filter<Extract=impl Reply, Error=Rejection> + Clone {
	pet_id()
		.and(warp::delete())
		.and(header("api_key"))
		.and(document::document(description("Deletes a pet")))
		.map(|id, _key: String| format!("Deleted pet #{}", id))
}

fn pet_json() -> impl Filter<Extract=(Pet, ), Error=Rejection> + Clone {
	warp::any()
		.and(body::json())
		.and(document::document(body(pet_struct()).mime("application/json")))
		.and(document::document(body(pet_struct()).mime("application/json")))
}

fn pet_post() -> impl Filter<Extract=impl Reply, Error=Rejection> + Clone {
	warp::post()
		.and(pet_json())
		.and(document::document(description("Adds a new pet to the store")))
		.map(|_| "Created")
}

fn pet_put() -> impl Filter<Extract=impl Reply, Error=Rejection> + Clone {
	warp::put()
		.and(pet_json())
		.and(document::document(description("Pet object that needs to be added to the store")))
		.map(|_| "Created")
}

fn pet_status() -> impl Filter<Extract=impl Reply, Error=Rejection> + Clone {
	path("findByStatus")
		.and(warp::get())
		.and(document::document(description("Finds pets by status")))
		.and(query())
		.and(document::document(|r: &mut RouteDocumentation| r.query(document::query("status", array(string())).required(true))))
		.and(document::document(response(200, body(array(pet_struct())).mime("application/json"))))
		.map(|_status: Vec<String>| reply::json(&vec![PetStatus::default(); 2]))
}

#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Pet {
	id: usize,
	category: Generic,
	name: String,
	photo_urls: Vec<String>,
	tags: Vec<Generic>,
	status: PetStatus,
}

#[derive(Clone, Default, Deserialize, Serialize)]
struct Generic {
	id: usize,
	name: String,
}

#[allow(dead_code)]
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum PetStatus {
	Available,
	Pending,
	Sold,
}

impl Default for PetStatus {
	fn default() -> Self {
		Self::Available
	}
}

/// Boiler Plate Ahead!
/// This will describe the pet struct we use.
///
/// In the future I hope to get rid of this via a derive macro e.g.
/// ```ignore
/// #[derive(DocumentedType)]
/// struct Pet {
///     id: usize,
///     category: Category,
///     /* etc. */
/// }
/// ```
fn pet_struct() -> DocumentedType {
	let mut properties = HashMap::with_capacity(6);
	properties.insert("id".into(), integer());
	properties.insert("category".into(), generic_struct());
	properties.insert("name".into(), string().example("Doggy".into()));
	properties.insert("photoUrls".into(), array(string()));
	properties.insert("tags".into(), array(generic_struct()));
	// Enums are not yet supported
	properties.insert("status".into(), string().description("The pet's status in the store"));
	properties.into()
}

fn generic_struct() -> DocumentedType {
	let mut properties = HashMap::with_capacity(2);
	properties.insert("id".into(), integer());
	properties.insert("name".into(), string());
	properties.into()
}
