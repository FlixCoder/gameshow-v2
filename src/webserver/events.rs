use std::{
	pin::Pin,
	task::{Context, Poll},
	time::Duration,
};

use actix_web::{error, get, rt, web, HttpRequest, HttpResponse, Result as HttpResult};
use futures::Stream;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

use super::ensure_cookie_consent;
use crate::{datahandler::DataHandler, game::Event};

const PING_INTERVAL: u64 = 10; //interval to ping clients in seconds

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(event_stream);
}

#[get("/{lobby_id}")]
async fn event_stream(
	db: web::Data<DataHandler>,
	request: HttpRequest,
	lobby_id: web::Path<String>,
) -> HttpResult<HttpResponse> {
	ensure_cookie_consent(&request)?;

	let db_lobby =
		db.get_lobby(lobby_id.into_inner()).await.map_err(error::ErrorInternalServerError)?;
	if db_lobby.is_some() {
		let lobby = db_lobby.unwrap();
		let client = EventStreamClient::new(lobby.subsribe_events().await);

		let mut resp = HttpResponse::Ok();
		resp.append_header(("Content-Type", "text/event-stream"))
			.append_header(("Cache-Control", "no-cache"));
		Ok(resp.streaming(client))
	} else {
		Err(error::ErrorNotFound("Lobby not found: Lobby UUID not in database!"))
	}
}

struct EventStreamClient {
	event_source: BroadcastStream<Event>,
	pinger: rt::time::Interval,
}

impl EventStreamClient {
	pub fn new(event_source: broadcast::Receiver<Event>) -> Self {
		EventStreamClient {
			event_source: BroadcastStream::new(event_source),
			pinger: rt::time::interval(Duration::from_secs(PING_INTERVAL)),
		}
	}

	fn ping() -> web::Bytes {
		web::Bytes::from("event: ping\ndata: \"ping\"\n\n")
	}

	fn event_to_bytes(event: Event) -> web::Bytes {
		let data = serde_json::to_string(&event).unwrap();
		web::Bytes::from(format!("event: game_event\ndata: {}\n\n", data))
	}
}

use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
impl Stream for EventStreamClient {
	type Item = Result<web::Bytes, error::Error>;

	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		//if let Poll::Ready(_) = Pin::new(&mut self.pinger).poll_next(cx) //will
		// register wakeup through cx.waker() on pending
		if self.pinger.poll_tick(cx).is_ready()
		//will register wakeup through cx.waker() on pending
		{
			Poll::Ready(Some(Ok(EventStreamClient::ping())))
		} else {
			match Pin::new(&mut self.event_source).poll_next(cx) //will register wakeup through cx.waker() on pending
            {
                Poll::Ready(Some(Ok(content))) => Poll::Ready(Some(Ok(EventStreamClient::event_to_bytes(content)))),
                Poll::Ready(None) => Poll::Ready(None), //event sender connection was closed
                Poll::Ready(Some(Err(BroadcastStreamRecvError::Lagged(_)))) => Poll::Ready(None), //close connection when messages were lost
                Poll::Pending => Poll::Pending,
            }
		}
	}
}
