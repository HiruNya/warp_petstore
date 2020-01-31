use warp::{body, Filter, filters::path::path, Rejection, reply::{self, Reply}};
use warp::document::{self, body, boolean, description, DocumentedType, integer, response, string};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::util::param;

pub fn store() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	path("store")
		.and(path("order"))
		.and(
			find_order()
			.or(delete_order())
			.or(place_order())
		)
}

fn order_id() -> impl Filter<Extract = (u32,), Error = Rejection> + Clone {
	param("orderId", "Id of pet that needs to be fetched")
		.and(document::document(response(404, None).description("Order not found")))
}

fn find_order() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
	order_id()
		.and(warp::get())
		.and(document::document(description("Find purchase order by ID")))
		.and(document::document(response(200, body(order_struct()).mime("application/json"))))
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
		.and(document::document(body(order_struct()).mime("application/json")))
		.and(document::document(response(200, body(order_struct()).mime("application/json")).description("A successful operation")))
		.map(|order: Order| reply::json(&order))
}

fn order_struct() -> DocumentedType {
	let mut properties = HashMap::with_capacity(6);
	properties.insert("id".into(), integer());
	properties.insert("petId".into(), integer());
	properties.insert("quantity".into(), integer());
	properties.insert("shipDate".into(), string());
	properties.insert("status".into(), string());
	properties.insert("complete".into(), boolean());
	properties.into()
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

