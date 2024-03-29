use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, RwLock};

mod events;
mod questions;
mod state;

pub use events::Event;
use events::*;
pub use questions::{find_question_files, Question, QuestionType};
use state::LobbyState;

//standard parameters for the game
const INITIAL_MONEY: i64 = 500; //initial amount of money every player owns
const INITIAL_JOKERS: usize = 3; //number of inital jokers every player gets
const NORMAL_Q_MONEY: i64 = 500; //money to get when answering a normal question correctly
const ESTIMATION_Q_MONEY: i64 = 1000; //money to get when winning a estimation question

//object for one gameshow lobby; includes all necessary data and methods to
// interact lock order to avoid deadlocks: admin -> open -> lobby_state ->
// question_set -> questions -> player_data -> game_events
pub struct Gameshow {
	//data related to lobby
	admin: RwLock<(String, String)>, //UUID and name of player that controls the lobby
	open: RwLock<bool>,              //whether or not the lobby accepts additional players
	param_initial_money: AtomicI64,  //see respective constants
	param_initial_jokers: AtomicUsize, //see respective constants
	param_normal_q_money: AtomicI64, //see respective constants
	param_estimation_q_money: AtomicI64, //see respective constants
	question_set: RwLock<String>,    //name of selected questions

	//data related to the game
	lobby_state: RwLock<LobbyState>,
	questions: RwLock<Vec<Question>>,
	current_question: AtomicUsize,
	player_data: RwLock<Vec<PlayerData>>,
	game_events: RwLock<EventManager>,
}

impl Gameshow {
	pub fn new(admin: String, name: String) -> Self {
		Gameshow {
			admin: RwLock::new((admin, name)),
			open: RwLock::new(true),
			param_initial_money: AtomicI64::new(INITIAL_MONEY),
			param_initial_jokers: AtomicUsize::new(INITIAL_JOKERS),
			param_normal_q_money: AtomicI64::new(NORMAL_Q_MONEY),
			param_estimation_q_money: AtomicI64::new(ESTIMATION_Q_MONEY),
			question_set: RwLock::new(String::new()),

			lobby_state: RwLock::new(LobbyState::Menu(false)),
			questions: RwLock::new(Vec::new()),
			current_question: AtomicUsize::new(0),
			player_data: RwLock::new(Vec::new()),
			game_events: RwLock::new(EventManager::new()),
		}
	}

	#[allow(dead_code)]
	async fn generate_player_update(&self) {
		//send PlayerListUpdate to clients
		let player_access = self.player_data.read().await;
		let event = EventType::PlayerListUpdate(EventPlayerListUpdate {
			player_data: make_public_player_data(&player_access),
		});
		self.game_events.write().await.add(event);
	}

	async fn generate_lobby_update(&self) {
		let lobby_open = self.is_open().await;
		let question_set = self.get_question_set().await;
		//send LobbySettingsUpdate to clients
		let event = EventType::LobbySettingsUpdate(EventLobbySettingsUpdate {
			open: lobby_open,
			initial_money: self.get_initial_money(),
			initial_jokers: self.get_initial_jokers(),
			normal_q_money: self.get_normal_q_money(),
			estimation_q_money: self.get_estimation_q_money(),
			question_set,
		});
		self.game_events.write().await.add(event);
	}

	pub async fn get_admin_uuid(&self) -> String {
		let admin_access = self.admin.read().await;
		admin_access.0.clone()
	}

	pub async fn get_admin_name(&self) -> String {
		let admin_access = self.admin.read().await;
		admin_access.1.clone()
	}

	#[allow(dead_code)]
	pub async fn set_admin(&self, admin: String, name: String) -> &Self {
		{
			let mut admin_access = self.admin.write().await;
			(*admin_access) = (admin, name);
		}
		self
	}

	pub async fn is_open(&self) -> bool {
		let open_access = self.open.read().await;
		*open_access
	}

	pub fn get_initial_money(&self) -> i64 {
		self.param_initial_money.load(Ordering::Relaxed)
	}

	pub fn get_initial_jokers(&self) -> usize {
		self.param_initial_jokers.load(Ordering::Relaxed)
	}

	pub fn get_normal_q_money(&self) -> i64 {
		self.param_normal_q_money.load(Ordering::Relaxed)
	}

	pub fn get_estimation_q_money(&self) -> i64 {
		self.param_estimation_q_money.load(Ordering::Relaxed)
	}

	pub async fn get_question_set(&self) -> String {
		let question_set_access = self.question_set.read().await;
		(*question_set_access).clone()
	}

	pub async fn set_open(&self, open: bool) -> &Self {
		{
			//set new preference
			let mut open_access = self.open.write().await;
			(*open_access) = open;
		}

		//send update event to clients
		self.generate_lobby_update().await;

		self
	}

	pub async fn update_preferences(
		&self,
		initial_money: i64,
		initial_jokers: usize,
		normal_q_money: i64,
		estimation_q_money: i64,
	) -> &Self {
		//ensure current lobby state is correct
		if *self.lobby_state.read().await != LobbyState::Menu(false) {
			return self;
		}

		if initial_money >= 0 {
			self.param_initial_money.store(initial_money, Ordering::Relaxed);
		}

		self.param_initial_jokers.store(initial_jokers, Ordering::Relaxed);

		if normal_q_money > 0 {
			self.param_normal_q_money.store(normal_q_money, Ordering::Relaxed);
		}

		if estimation_q_money > 0 {
			self.param_estimation_q_money.store(estimation_q_money, Ordering::Relaxed);
		}

		//send update event to clients
		self.generate_lobby_update().await;

		self
	}

	pub async fn set_question_set(&self, question_set: &str) -> std::io::Result<&Self> {
		//ensure current lobby state is correct
		if *self.lobby_state.read().await != LobbyState::Menu(false) {
			return Ok(self);
		}

		//preload question set (if not custom)
		let mut questions = Vec::new();
		if question_set != "custom" {
			let question_sets = questions::find_question_files()?;
			for (name, file) in question_sets.iter() {
				if name == question_set {
					questions = questions::read_questions(file)?;
					break;
				}
			}
		}

		{
			//update preference and save questions (if not custom)
			let mut question_set_access = self.question_set.write().await;
			(*question_set_access) = String::from(question_set);

			if (*question_set_access) != "custom" {
				self.current_question.store(0, Ordering::Relaxed);
				let mut questions_access = self.questions.write().await;
				(*questions_access) = questions;
			}
		}

		//send update event to clients
		self.generate_lobby_update().await;

		Ok(self)
	}

	pub async fn set_questions(&self, questions: Vec<Question>) -> std::io::Result<&Self> {
		//ensure current lobby state is correct and question_set allows custom
		// questions
		if *self.lobby_state.read().await != LobbyState::Menu(false)
			|| *self.question_set.read().await != "custom"
		{
			return Ok(self);
		}

		self.current_question.store(0, Ordering::Relaxed);
		let mut questions_access = self.questions.write().await;
		(*questions_access) = questions;

		Ok(self)
	}

	pub async fn get_player_data(&self) -> Vec<PublicPlayerData> {
		make_public_player_data(&self.player_data.read().await)
	}

	pub async fn get_events(&self) -> Vec<Event> {
		self.game_events.read().await.get()
	}

	pub async fn subsribe_events(&self) -> broadcast::Receiver<Event> {
		self.game_events.read().await.subscribe()
	}

	#[allow(dead_code)]
	pub async fn get_event_subscribers(&self) -> usize {
		self.game_events.read().await.get_subscribers()
	}

	pub async fn join(&self, uuid: &str, mut name: String) -> Option<String> {
		//check if already joined and return true if so; also check if name is already
		// in use
		let mut name_in_use = false;
		{
			let player_access = self.player_data.read().await;
			for player in (*player_access).iter() {
				if player.uuid == uuid {
					return Some(player.name.clone());
				} else if player.name == name {
					name_in_use = true;
				}
			}
		}

		//if not already joined, check if allowed to join
		if self.get_admin_uuid().await == uuid {
			//admin can always join with its name
			let mut player_access = self.player_data.write().await;
			let new_player = PlayerData {
				uuid: String::from(uuid),
				name: name.clone(),
				jokers: self.get_initial_jokers(),
				money: self.get_initial_money(),

				money_bet: 0,
				vs_player: String::new(),
				answer: 0,
			};
			(*player_access).push(new_player);
			//send PlayerListUpdate to clients
			let event = EventType::PlayerListUpdate(EventPlayerListUpdate {
				player_data: make_public_player_data(&player_access),
			});
			self.game_events.write().await.add(event);
			//return name
			Some(name)
		} else if self.is_open().await {
			//others need to have unique name (from others and from admin)
			let admin_name = self.get_admin_name().await;
			let mut player_access = self.player_data.write().await;
			//make sure the name is unique
			while name_in_use || name == admin_name {
				name += &generate_random_string(2);
				name_in_use = false;
				for player in (*player_access).iter() {
					if player.name == name {
						name_in_use = true;
						break;
					}
				}
			}
			let new_player = PlayerData {
				uuid: String::from(uuid),
				name: name.clone(),
				jokers: self.get_initial_jokers(),
				money: self.get_initial_money(),

				money_bet: 0,
				vs_player: String::new(),
				answer: 0,
			};
			(*player_access).push(new_player);
			//send PlayerListUpdate to clients
			let event = EventType::PlayerListUpdate(EventPlayerListUpdate {
				player_data: make_public_player_data(&player_access),
			});
			self.game_events.write().await.add(event);
			//return name
			Some(name)
		} else {
			None
		}
	}

	pub async fn leave(&self, uuid: &str) -> bool {
		let contained;
		{
			let mut player_access = self.player_data.write().await;
			contained = (*player_access).iter().any(|player| player.uuid == uuid);
			if contained {
				(*player_access).retain(|player| player.uuid != uuid);
				//send PlayerListUpdate to clients
				let event = EventType::PlayerListUpdate(EventPlayerListUpdate {
					player_data: make_public_player_data(&player_access),
				});
				self.game_events.write().await.add(event);
			}
		}

		self.state_transition().await;
		contained

		//in the future when drain_filter is not experimental anymore
		//let removed = (*player_access).drain_filter(|player| player.uuid !=
		// uuid); let contained = removed.count() != 0;
	}

	pub async fn kick_player(&self, name: &str) -> bool {
		let contained;
		{
			let mut player_access = self.player_data.write().await;
			contained = (*player_access).iter().any(|player| player.name == name);
			if contained {
				(*player_access).retain(|player| player.name != name);
				//send PlayerListUpdate to clients
				let event = EventType::PlayerListUpdate(EventPlayerListUpdate {
					player_data: make_public_player_data(&player_access),
				});
				self.game_events.write().await.add(event);
			}
		}

		self.state_transition().await;
		contained

		//in the future when drain_filter is not experimental anymore
		//let removed = (*player_access).drain_filter(|player| player.name !=
		// name); let contained = removed.count() != 0;
	}

	pub async fn set_player_attributes(&self, name: &str, money: i64, jokers: usize) -> bool {
		let mut player_access = self.player_data.write().await;
		let mut contained = false;
		(*player_access).iter_mut().for_each(|player| {
			if player.name == name {
				player.money = money;
				player.jokers = jokers;
				contained = true;
			}
		});
		if contained {
			//send PlayerListUpdate to clients
			let event = EventType::PlayerListUpdate(EventPlayerListUpdate {
				player_data: make_public_player_data(&player_access),
			});
			self.game_events.write().await.add(event);
		}
		contained
	}

	pub async fn is_joined(&self, uuid: &str) -> bool {
		self.player_data.read().await.iter().any(|player| player.uuid == uuid)
	}

	pub async fn get_player_name(&self, uuid: &str) -> Option<String> {
		for player in self.player_data.read().await.iter() {
			if player.uuid == uuid {
				return Some(player.name.clone());
			}
		}
		None
	}

	pub async fn get_player_money(&self, uuid: &str) -> Option<i64> {
		for player in self.player_data.read().await.iter() {
			if player.uuid == uuid {
				return Some(player.money);
			}
		}
		None
	}

	pub async fn get_player_jokers(&self, uuid: &str) -> Option<usize> {
		for player in self.player_data.read().await.iter() {
			if player.uuid == uuid {
				return Some(player.jokers);
			}
		}
		None
	}

	pub async fn is_valid_vs_player(&self, vs_player: &str) -> bool {
		for player in self.player_data.read().await.iter() {
			if player.name == vs_player {
				return true;
			}
		}
		false
	}

	pub async fn bet(&self, uuid: &str, money_bet: i64) -> bool {
		//ensure current lobby state is correct
		if *self.lobby_state.read().await != LobbyState::BettingQBetting(false) {
			return false;
		}

		let mut all_bet = true;
		{
			//perform money betting and check if all players have bet
			let mut player_access = self.player_data.write().await;
			for player in (*player_access).iter_mut() {
				if player.uuid == uuid {
					//set player's money_bet
					player.money_bet = money_bet;
				} else if player.money_bet < 1 {
					//check if player has bet
					all_bet = false;
				}
			}

			//send PlayerListUpdate to clients
			let event = EventType::PlayerListUpdate(EventPlayerListUpdate {
				player_data: make_public_player_data(&player_access),
			});
			self.game_events.write().await.add(event);
		}

		//indicate abilitiy to proceed when all players bet
		if all_bet {
			let mut state = self.lobby_state.write().await;
			*state = LobbyState::BettingQBetting(true);
		}

		self.state_transition().await;
		true
	}

	pub async fn attack(&self, uuid: &str, vs_player: &str) -> bool {
		//ensure current lobby state is correct
		if *self.lobby_state.read().await != LobbyState::VersusQSelecting(false) {
			return false;
		}

		let mut all_selected = true;
		{
			//perform player selecting and check if all players have selected
			let mut player_access = self.player_data.write().await;
			for player in (*player_access).iter_mut() {
				if player.uuid == uuid {
					//set player's vs_player
					player.vs_player = String::from(vs_player);
				} else if player.vs_player.is_empty() {
					//check if player has selected
					all_selected = false;
				}
			}

			//send PlayerListUpdate to clients
			let event = EventType::PlayerListUpdate(EventPlayerListUpdate {
				player_data: make_public_player_data(&player_access),
			});
			self.game_events.write().await.add(event);
		}

		//indicate abilitiy to proceed when all players selected
		if all_selected {
			let mut state = self.lobby_state.write().await;
			*state = LobbyState::VersusQSelecting(true);
		}

		self.state_transition().await;
		true
	}

	pub async fn answer(&self, uuid: &str, answer: usize) -> bool {
		//ensure current lobby state is correct
		{
			let state = self.lobby_state.read().await;
			if *state != LobbyState::NormalQAnswering(false)
				&& *state != LobbyState::BettingQAnswering(false)
				&& *state != LobbyState::EstimationQAnswering(false)
				&& *state != LobbyState::VersusQAnswering(false)
			{
				return false;
			}
		}

		let mut all_answered = true;
		{
			//perform answering and check if all players have answered
			let mut player_access = self.player_data.write().await;
			for player in (*player_access).iter_mut() {
				if player.uuid == uuid {
					//set player's answer
					player.answer = answer;
				} else if player.answer < 1 {
					//check if player has answered
					all_answered = false;
				}
			}

			//send PlayerListUpdate to clients
			let event = EventType::PlayerListUpdate(EventPlayerListUpdate {
				player_data: make_public_player_data(&player_access),
			});
			self.game_events.write().await.add(event);
		}

		//indicate abilitiy to proceed when all players bet
		if all_answered {
			let mut state = self.lobby_state.write().await;
			*state = match *state {
				LobbyState::NormalQAnswering(_) => LobbyState::NormalQAnswering(true),
				LobbyState::BettingQAnswering(_) => LobbyState::BettingQAnswering(true),
				LobbyState::EstimationQAnswering(_) => LobbyState::EstimationQAnswering(true),
				LobbyState::VersusQAnswering(_) => LobbyState::VersusQAnswering(true),
				default => default,
			};
		}

		self.state_transition().await;
		true
	}

	pub async fn get_joker(&self, uuid: &str) -> Option<Vec<usize>> {
		//ensure current lobby state is correct
		{
			let state = self.lobby_state.read().await;
			if *state != LobbyState::NormalQAnswering(false)
				&& *state != LobbyState::BettingQAnswering(false)
			{
				return None;
			}
		}

		//get wrong answers
		let wrong_answers: Vec<usize>;
		{
			let mut rng = rand::thread_rng();
			let current_question = self.current_question.load(Ordering::Relaxed);
			let questions_access = self.questions.read().await;
			let correct_answer = (*questions_access)[current_question - 1].correct_answer;
			let mut choose_from = vec![1, 2, 3, 4];
			choose_from.remove(correct_answer - 1); //removed by index
			wrong_answers = choose_from.choose_multiple(&mut rng, 2).copied().collect();
		}

		//decrement player's jokers wrong answers
		let mut player_access = self.player_data.write().await;
		for player in (*player_access).iter_mut() {
			if player.uuid == uuid {
				player.jokers -= 1;
				break;
			}
		}

		//send PlayerListUpdate to clients
		let event = EventType::PlayerListUpdate(EventPlayerListUpdate {
			player_data: make_public_player_data(&player_access),
		});
		self.game_events.write().await.add(event);

		//send/return wrong answers as joker
		Some(wrong_answers)
	}

	pub async fn next_state(&self) {
		state::initiate_next(&self.lobby_state).await;
		self.state_transition().await;
	}

	async fn state_transition(&self) {
		let mut repeat = true;
		while repeat {
			repeat = !state::state_transition(self).await;
		}
	}
}

//struct for player data
#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerData {
	uuid: String,
	name: String,
	jokers: usize,
	money: i64,
	//could also use Option<>, but easier for frontend to handle without
	money_bet: i64,
	vs_player: String,
	answer: usize,
}

//struct for player data to be sent to clients (without uuid)
#[derive(Serialize, Deserialize, Clone)]
pub struct PublicPlayerData {
	name: String,
	jokers: usize,
	money: i64,
	//could also use Option<>, but easier for frontend to handle without
	money_bet: i64,
	vs_player: String,
	answer: usize,
}

fn make_public_player_data(players: &[PlayerData]) -> Vec<PublicPlayerData> {
	players
		.iter()
		.map(|player| PublicPlayerData {
			name: player.name.clone(),
			jokers: player.jokers,
			money: player.money,

			money_bet: player.money_bet,
			vs_player: player.vs_player.clone(),
			answer: player.answer,
		})
		.collect()
}

fn generate_random_string(length: usize) -> String {
	let characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                    abcdefghijklmnopqrstuvwxyz\
                    0123456789-_";
	let choose_from: Vec<char> = characters.chars().collect();

	let mut rng = rand::thread_rng();
	let mut rnd_string = String::from("");
	for _ in 0..length {
		rnd_string.push(choose_from.choose(&mut rng).unwrap().to_owned());
	}
	rnd_string
}
