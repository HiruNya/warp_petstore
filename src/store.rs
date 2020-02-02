use warp::{body, Filter, filters::path::path, Rejection, reply::{self, Reply}};
use warp::document::{self, body, boolean, description, DocumentedType, map, integer, response, string, ToDocumentedType};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::util::param;

pub fn store() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	path("store")
		.and(
			inventory()
			.or(
				path("order")
				.and(
					find_order()
					.or(delete_order())
					.or(place_order())
				)
			)
		)
}

fn inventory() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	path("inventory")
		.and(warp::get())
		.and(document::document(description("Returns pet inventories by status")))
		.and(document::document(response(200, body(map(integer())).mime("application/json")).description("Successful Operation")))
		.map(|| {
			let mut inventory = HashMap::with_capacity(2);
			inventory.insert("Dog", 3);
			inventory.insert("Cat", 1);
			reply::json(&inventory)
		})
}

fn order_id() -> impl Filter<Extract = (u32,), Error = Rejection> + Clone {
	param("orderId", "Id of pet that needs to be fetched")
		.and(document::document(response(404, None).description("Order not found")))
}

fn find_order() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
	order_id()
		.and(warp::get())
		.and(document::document(description("Find purchase order by ID")))
		.and(document::document(response(200, body(Order::document()).mime("application/json"))))
		.map(|id| reply::json(&Order{ id, ..Order::default() }))
}

fn delete_order() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	order_id()
		.and(warp::delete())
		.and(document::document(description("Delete purchase order by ID")))
		.and(document::document(response(200, None)))
		.map(|_id| "Deleted")
}

fn place_order() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	warp::post()
		.and(document::document(description("Place an order for a pet")))
		.and(body::json())
		.and(document::document(body(Order::document()).mime("application/json")))
		.and(document::document(response(200, body(Order::document()).mime("application/json")).description("A successful operation")))
		.map(|order: Order| reply::json(&order))
}

#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
	id: u32,
	pet_id: usize,
	quantity: u16,
	ship_date: String,
	status: String,
	complete: bool,
}

impl ToDocumentedType for Order {
	fn document() -> DocumentedType {
		let mut properties = HashMap::with_capacity(6);
		properties.insert("id".into(), integer());
		properties.insert("petId".into(), integer());
		properties.insert("quantity".into(), integer());
		properties.insert("shipDate".into(), string());
		properties.insert("status".into(), string());
		properties.insert("complete".into(), boolean());
		properties.into()
	}
}

