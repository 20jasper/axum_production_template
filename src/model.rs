//! Model layer with mock store layer for prototyping

use std::sync::{Arc, Mutex};

use crate::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct Ticket {
	id: u64,
	title: String,
}

#[derive(Deserialize)]
pub struct ClientTicket {
	title: String,
}

pub struct ModelController {
	ticket_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
	pub async fn new(ticket_store: Arc<Mutex<Vec<Option<Ticket>>>>) -> Result<Self> {
		Ok(Self { ticket_store })
	}
}
