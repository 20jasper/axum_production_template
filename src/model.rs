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

impl ModelController {
	pub async fn create_ticket(&self, ClientTicket { title }: ClientTicket) -> Result<Ticket> {
		let mut store = self.ticket_store.lock().unwrap();
		let id = store.len() as u64;

		let ticket = Ticket { id, title };

		store.push(Some(ticket.clone()));

		Ok(ticket)
	}

	pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
		let store = self.ticket_store.lock().unwrap();

		let tickets = store
			.iter()
			.filter_map(Option::as_ref)
			.cloned()
			.collect::<Vec<Ticket>>();

		Ok(tickets)
	}
}
