use warp::{body, Filter, path::path, Rejection, reply::{self, Reply}};
use warp::document::{self, body, description, DocumentedType, header, integer, response,
                     string, query, RouteDocumentation, ToDocumentedType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::util::param;

pub fn user() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	path("user")
		.and(
			login()
			.or(logout())
			.or(get_user())
			.or(update_user())
			.or(delete_user())
			.or(
				// For some reason in the official specification, the route is repeated with different names
				path("createWithArray").and(create_user_with_array())
				.or(path("createWithList").and(create_user_with_array()))
			)
			.or(create_user())
		)
}

fn username() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
	param("username", "The name that needs to be fetched")
		.and(document::document(response(404, None).description("User not found")))
}

fn get_user() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	warp::get()
		.and(username())
		.and(document::document(description("Get user by user name")))
		.and(document::document(response(200, body(User::document()).mime("application/json")).description("Successful operation")))
		.map(|username| reply::json(&User{ username, ..User::default() }))
}

fn update_user() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	warp::put()
		.and(username())
		.and(user_json())
		.and(document::document(description("Update a user")))
		.map(|name, _user: User| format!("Updated user: {}", name))
}

fn delete_user() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	warp::delete()
		.and(username())
		.and(document::document(description("Delete a user")))
		.map(|name| format!("Deleted: {}", name))
}

fn login() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	warp::get()
		.and(path("login"))
		.and(document::document(description("Logs the user into the system")))
		.and(document::document(|route: &mut RouteDocumentation| {
			route.query(query("username", string().description("The username for login")));
			route.query(query("password", string().description("The password for login in clear text")));
		}))
		.and(document::document(
			response(200, body(string()))
				.header(header("X-Expires-After").description("Date in UTC when token expires"))
				.header(header("X-Rate-Limit").description("Calls per hour allowed by the user"))))
		.and(document::document(response(400, None).description("Invalid username/password supplied")))
		.map(|| "Logged in!")
}

fn logout() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	warp::get()
		.and(path("logout"))
		.and(document::document(description("Logs out current logged in user session")))
		.map(|| "You have been logged out!")
}

fn create_user() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	warp::post()
		.and(document::document(description("Creates a user")))
		.and(user_json())
		.map(|user: User| format!("Created: {}!", user.username))
}

fn create_user_with_array() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
	warp::post()
		.and(document::document(description("Creates a list of users at once with a given array")))
		.and(body::json())
		.and(document::document(body(<Vec<User>>::document().description("List of user objects")).mime("application/json")))
		.map(|list: Vec<User>| format!("Created {} users.", list.len()))
}

fn user_json() -> impl Filter<Extract = (User,), Error = Rejection> + Clone {
	body::json()
		.and(document::document(body(User::document()).mime("application/json")))
}

#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct User {
	id: u32,
	username: String,
	first_name: String,
	last_name: String,
	email: String,
	password: String,
	phone: String,
	user_status: u8,
}

impl ToDocumentedType for User {
	fn document() -> DocumentedType {
		let mut map = HashMap::with_capacity(8);
		map.insert("id".into(), integer());
		map.insert("username".into(), string());
		map.insert("firstName".into(), string());
		map.insert("lastName".into(), string());
		map.insert("email".into(), string());
		map.insert("password".into(), string());
		map.insert("phone".into(), string());
		map.insert("userStatus".into(), integer());
		map.into()
	}
}
