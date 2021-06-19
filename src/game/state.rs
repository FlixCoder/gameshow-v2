use std::sync::atomic::{Ordering, AtomicUsize, AtomicI64};
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;

use super::{PlayerData, make_public_player_data};
use super::questions::{Question, QuestionType};
use super::events::*;


#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum LobbyState
{ //the bool indicates if it is ready to transition to next state
    Menu(bool),
    Results(bool),
    NormalQAnswering(bool),
    BettingQBetting(bool),
    BettingQAnswering(bool),
    EstimationQAnswering(bool),
    VersusQSelecting(bool),
    VersusQAnswering(bool),
    GameEnding(bool),
}


//allow to go to the next state without checking any conditions
pub async fn initiate_next(lobby_state: &RwLock<LobbyState>)
{
    let mut state = lobby_state.write().await;
    *state = match *state
    {
        LobbyState::Menu(false) => LobbyState::Menu(true),
        LobbyState::Results(false) => LobbyState::Results(true),
        LobbyState::NormalQAnswering(false) => LobbyState::NormalQAnswering(true),
        LobbyState::BettingQBetting(false) => LobbyState::BettingQBetting(true),
        LobbyState::BettingQAnswering(false) => LobbyState::BettingQAnswering(true),
        LobbyState::EstimationQAnswering(false) => LobbyState::EstimationQAnswering(true),
        LobbyState::VersusQSelecting(false) => LobbyState::VersusQSelecting(true),
        LobbyState::VersusQAnswering(false) => LobbyState::VersusQAnswering(true),
        LobbyState::GameEnding(false) => LobbyState::GameEnding(true),
        default => default,
    };
}

//check if next question state is possible/initiated and transition
//(by preparing everything and adding an event)
pub async fn state_transition(
    lobby_state: &RwLock<LobbyState>,
    questions: &RwLock<Vec<Question>>,
    current_question: &AtomicUsize,
    player_data: &RwLock<Vec<PlayerData>>,
    game_events: &RwLock<EventManager>,
    lobby_open: &RwLock<bool>,
    param_initial_money: &AtomicI64,
    param_initial_jokers: &AtomicUsize,
    param_normal_q_money: &AtomicI64,
    param_estimation_q_money: &AtomicI64,
) -> bool //returns if finished (false=>repeat)
{
    let mut state = lobby_state.write().await;
    match *state
    {
        LobbyState::Menu(true) => { //transition to first question (different states for different questions)
            //prepare lobby, start question by setting state to LobbyState::Results(true) and transitioning again
            current_question.store(0, Ordering::Relaxed);
            let mut player_access = player_data.write().await;
            for player in (*player_access).iter_mut()
            { //reset every player to starting conditions
                player.money = param_initial_money.load(Ordering::Relaxed);
                player.jokers = param_initial_jokers.load(Ordering::Relaxed);
            }
            //create event (not for state transition yet)
            let new_event = EventType::PlayerListUpdate(EventPlayerListUpdate {
                player_data: make_public_player_data(&*player_access),
            });
            game_events.write().await.add(new_event);
            //set new question state and return false to repeat
            *state = LobbyState::Results(true);
            return false;
        },
        LobbyState::Results(true) => { //transition to next question (different states for different questions)
            //gather necessary data
            let question_id = current_question.fetch_add(1, Ordering::Relaxed) + 1;
            let questions = questions.read().await;
            let num_questions = (*questions).len();
            if question_id > num_questions
            { //game ending - create event
                let new_event = EventType::GameEnding(EventGameEnding {
                    player_data: make_public_player_data(&*player_data.read().await)
                });
                game_events.write().await.add(new_event);
                //set new question state
                *state = LobbyState::GameEnding(false);
            }
            else
            { //next question
                let question_type = (*questions)[question_id - 1].question_type.clone();
                let category = (*questions)[question_id - 1].category.clone();
                let question = (*questions)[question_id - 1].question.clone();
                let answers = (*questions)[question_id - 1].answers.clone();
                //reset bets and question answers for all players
                let mut player_access = player_data.write().await;
                for player in (*player_access).iter_mut()
                { //change zeros to None when using Options
                    player.money_bet = 0;
                    player.vs_player = "".to_owned();
                    player.answer = 0;
                }
                //depending on question type begin different question-specific event
                match question_type
                {
                    QuestionType::NormalQuestion => {
                        let new_event = EventType::BeginNormalQAnswering(EventBeginNormalQAnswering {
                            question_type: question_type,
                            current_question: question_id,
                            category: category,
                            question: question,
                            answers: answers
                        });
                        game_events.write().await.add(new_event);
                        //set new question state
                        *state = LobbyState::NormalQAnswering(false);
                    },
                    QuestionType::BettingQuestion => {
                        let new_event = EventType::BeginBettingQBetting(EventBeginBettingQBetting {
                            question_type: question_type,
                            current_question: question_id,
                            category: category
                        });
                        game_events.write().await.add(new_event);
                        //set new question state
                        *state = LobbyState::BettingQBetting(false);
                    },
                    QuestionType::EstimationQuestion => {
                        let new_event = EventType::BeginEstimationQAnswering(EventBeginEstimationQAnswering {
                            question_type: question_type,
                            current_question: question_id,
                            category: category,
                            question: question
                        });
                        game_events.write().await.add(new_event);
                        //set new question state
                        *state = LobbyState::EstimationQAnswering(false);
                    },
                    QuestionType::VersusQuestion => {
                        let new_event = EventType::BeginVersusQSelecting(EventBeginVersusQSelecting {
                            question_type: question_type,
                            current_question: question_id,
                            category: category
                        });
                        game_events.write().await.add(new_event);
                        //set new question state
                        *state = LobbyState::VersusQSelecting(false);
                    },
                }
            }
        },
        LobbyState::BettingQBetting(true) => { //transition to answering state
            //gather necessary data
            let question_id = current_question.load(Ordering::Relaxed);
            let questions = questions.read().await;
            let question = (*questions)[question_id - 1].question.clone();
            let answers = (*questions)[question_id - 1].answers.clone();
            //create event
            let new_event = EventType::BeginBettingQAnswering(EventBeginBettingQAnswering {
                question: question,
                answers: answers
            });
            game_events.write().await.add(new_event);
            //set new question state
            *state = LobbyState::BettingQAnswering(false);
        },
        LobbyState::VersusQSelecting(true) => { //transition to answering state
            //gather necessary data
            let question_id = current_question.load(Ordering::Relaxed);
            let questions = questions.read().await;
            let question = (*questions)[question_id - 1].question.clone();
            let answers = (*questions)[question_id - 1].answers.clone();
            //create event
            let new_event = EventType::BeginVersusQAnswering(EventBeginVersusQAnswering {
                question: question,
                answers: answers
            });
            game_events.write().await.add(new_event);
            //set new question state
            *state = LobbyState::VersusQAnswering(false);
        },
        LobbyState::NormalQAnswering(true) => { //transition to results state
            //gather necessary data
            let question_id = current_question.load(Ordering::Relaxed);
            let questions = questions.read().await;
            let correct_answer = (*questions)[question_id - 1].correct_answer;
            //compute the new money of each player
            let mut player_access = player_data.write().await;
            let previous_player_data = (*player_access).clone();
            for player in (*player_access).iter_mut()
            {
                if player.answer == correct_answer
                {
                    let normal_q_money = param_normal_q_money.load(Ordering::Relaxed);
                    player.money += normal_q_money;
                }
            }
            //create event
            let new_event = EventType::ShowResults(EventShowResults {
                correct_answer: correct_answer,
                previous_player_data: make_public_player_data(&previous_player_data),
                player_data: make_public_player_data(&*player_access)
            });
            game_events.write().await.add(new_event);
            //set new question state
            *state = LobbyState::Results(false);
        },
        LobbyState::BettingQAnswering(true) => { //transition to results state
            //gather necessary data
            let question_id = current_question.load(Ordering::Relaxed);
            let questions = questions.read().await;
            let correct_answer = (*questions)[question_id - 1].correct_answer;
            //compute the new money of each player
            let mut player_access = player_data.write().await;
            let previous_player_data = (*player_access).clone();
            for player in (*player_access).iter_mut()
            {
                if player.answer == correct_answer
                {
                    player.money += player.money_bet;
                }
                else
                {
                    player.money -= player.money_bet;
                    //if player has no money, give 1€ to allow continuing the game
                    if player.money == 0
                    {
                        player.money = 1;
                    }
                }
            }
            //create event
            let new_event = EventType::ShowResults(EventShowResults {
                correct_answer: correct_answer,
                previous_player_data: make_public_player_data(&previous_player_data),
                player_data: make_public_player_data(&*player_access)
            });
            game_events.write().await.add(new_event);
            //set new question state
            *state = LobbyState::Results(false);
        },
        LobbyState::EstimationQAnswering(true) => { //transition to results state
            //gather necessary data
            let question_id = current_question.load(Ordering::Relaxed);
            let questions = questions.read().await;
            let correct_answer = (*questions)[question_id - 1].correct_answer;
            //compute the new money of each player
            let mut closest_players = Vec::new();
            let mut min_dinstance = usize::MAX;
            let mut player_access = player_data.write().await;
            let previous_player_data = (*player_access).clone();
            for player in (*player_access).iter()
            {
                let diff = if player.answer >= correct_answer { player.answer - correct_answer } else { correct_answer - player.answer };
                if diff < min_dinstance
                {
                    min_dinstance = diff;
                    closest_players = vec![player.name.clone()];
                }
                else if diff == min_dinstance
                {
                    closest_players.push(player.name.clone());
                }
            }
            for player in (*player_access).iter_mut()
            {
                if closest_players.iter().any(|name| name == &player.name)
                {
                    let estimation_q_money = param_estimation_q_money.load(Ordering::Relaxed);
                    player.money += estimation_q_money;
                }
            }
            //create event
            let new_event = EventType::ShowResults(EventShowResults {
                correct_answer: correct_answer,
                previous_player_data: make_public_player_data(&previous_player_data),
                player_data: make_public_player_data(&*player_access)
            });
            game_events.write().await.add(new_event);
            //set new question state
            *state = LobbyState::Results(false);
        },
        LobbyState::VersusQAnswering(true) => { //transition to results state
            //gather necessary data
            let question_id = current_question.load(Ordering::Relaxed);
            let questions = questions.read().await;
            let correct_answer = (*questions)[question_id - 1].correct_answer;
            //compute the new money of each player
            let mut player_access = player_data.write().await;
            let previous_player_data = (*player_access).clone();
            let num_players = (*player_access).len();
            let mut player_factors: Vec<f64> = vec![1.0; num_players];
            for i in 0 .. num_players
            {
                if (*player_access)[i].vs_player.is_empty() { continue; }
                for j in 0 .. num_players
                {
                    if (*player_access)[i].vs_player == (*player_access)[j].name
                    {
                        if (*player_access)[i].answer == correct_answer
                        {
                            //player_factors[i] *= 2.0;
                            player_factors[j] /= 2.0;
                        }
                        else
                        {
                            //player_factors[i] /= 2.0;
                            player_factors[j] *= 2.0;
                        }
                        break;
                    }
                }
            }
            for i in 0 .. num_players
            {
                (*player_access)[i].money = ((*player_access)[i].money as f64 * player_factors[i]) as i64;
                //if player has no money, give 1€ to allow continuing the game
                if (*player_access)[i].money == 0
                {
                    (*player_access)[i].money = 1;
                }
            }
            //create event
            let new_event = EventType::ShowResults(EventShowResults {
                correct_answer: correct_answer,
                previous_player_data: make_public_player_data(&previous_player_data),
                player_data: make_public_player_data(&*player_access)
            });
            game_events.write().await.add(new_event);
            //set new question state
            *state = LobbyState::Results(false);
        },
        LobbyState::GameEnding(true) => { //transition to menu state, no need to do anything else
            //create event
            let new_event = EventType::BackToMenu(EventBackToMenu {
                open: *lobby_open.read().await
            });
            game_events.write().await.add(new_event);
            //set new question state
            *state = LobbyState::Menu(false);
        },
        
        LobbyState::Menu(false) |
        LobbyState::Results(false) |
        LobbyState::NormalQAnswering(false) |
        LobbyState::BettingQBetting(false) |
        LobbyState::BettingQAnswering(false) |
        LobbyState::EstimationQAnswering(false) |
        LobbyState::VersusQSelecting(false) |
        LobbyState::VersusQAnswering(false) |
        LobbyState::GameEnding(false) => {},
    }
    true
}
