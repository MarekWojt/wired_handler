use hyper::body::Bytes;
use hyper_tungstenite::tungstenite::Message;
use serde::{Deserialize, Serialize};

use crate::{
    data::request_body::GetBodyError,
    prelude::*,
    state::{
        connection_state::ConnectionState, context::WebsocketRequestContext,
        global_state::GlobalState, request_state::RequestState, session_state::SessionState,
    },
};

#[test]
fn test() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(run_test());
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UnusedType;

async fn run_test() {
    let global_state = GlobalState::default();
    let session_state = SessionState::default();
    let connection_state = ConnectionState::default();

    {
        let person = Person {
            name: "Franz".to_string(),
            age: 81,
        };
        let mut request_state = RequestState::default();
        request_state.insert(Message::binary(serde_json::to_vec(&person).unwrap()));

        let mut context = WebsocketRequestContext::from_states(
            global_state.clone(),
            session_state.clone(),
            connection_state.clone(),
            request_state,
        );

        assert_eq!(context.body::<Person>().await.unwrap(), &person);
        assert_eq!(context.body_mut::<Person>().await.unwrap(), &person);
        assert!(matches!(
            context.body::<UnusedType>().await,
            Err(GetBodyError::AlreadyParsed)
        ));
        assert_eq!(context.body::<Person>().await.unwrap(), &person);
        assert_eq!(context.remove_body::<Person>().await.unwrap(), person);
        assert!(matches!(
            context.body::<Person>().await,
            Err(GetBodyError::AlreadyParsed)
        ));
    }

    {
        let person = Person {
            name: "Franz".to_string(),
            age: 81,
        };
        let mut request_state = RequestState::default();
        request_state.insert(Message::text(serde_json::to_string(&person).unwrap()));

        let mut context = WebsocketRequestContext::from_states(
            global_state.clone(),
            session_state.clone(),
            connection_state.clone(),
            request_state,
        );

        assert_eq!(context.body::<Person>().await.unwrap(), &person);
        assert_eq!(context.body_mut::<Person>().await.unwrap(), &person);
        assert!(matches!(
            context.body::<UnusedType>().await,
            Err(GetBodyError::AlreadyParsed)
        ));
        assert_eq!(context.body::<Person>().await.unwrap(), &person);
        assert_eq!(context.remove_body::<Person>().await.unwrap(), person);
        assert!(matches!(
            context.body::<Person>().await,
            Err(GetBodyError::AlreadyParsed)
        ));
    }

    {
        let mut request_state = RequestState::default();
        request_state.insert(Message::Close(None));

        let mut context = WebsocketRequestContext::from_states(
            global_state.clone(),
            session_state.clone(),
            connection_state.clone(),
            request_state,
        );

        assert!(matches!(
            context.body::<Person>().await,
            Err(GetBodyError::InvalidMessageType)
        ));
        assert!(matches!(
            context.body_mut::<Person>().await,
            Err(GetBodyError::InvalidMessageType)
        ));
        assert!(matches!(
            context.remove_body::<Person>().await,
            Err(GetBodyError::InvalidMessageType)
        ));
    }

    {
        let mut request_state = RequestState::default();
        request_state.insert(Message::Ping(Bytes::new()));

        let mut context = WebsocketRequestContext::from_states(
            global_state.clone(),
            session_state.clone(),
            connection_state.clone(),
            request_state,
        );

        assert!(matches!(
            context.body::<Person>().await,
            Err(GetBodyError::InvalidMessageType)
        ));
        assert!(matches!(
            context.body_mut::<Person>().await,
            Err(GetBodyError::InvalidMessageType)
        ));
        assert!(matches!(
            context.remove_body::<Person>().await,
            Err(GetBodyError::InvalidMessageType)
        ));
    }

    {
        let mut request_state = RequestState::default();
        request_state.insert(Message::Pong(Bytes::new()));

        let mut context = WebsocketRequestContext::from_states(
            global_state.clone(),
            session_state.clone(),
            connection_state.clone(),
            request_state,
        );

        assert!(matches!(
            context.body::<Person>().await,
            Err(GetBodyError::InvalidMessageType)
        ));
        assert!(matches!(
            context.body_mut::<Person>().await,
            Err(GetBodyError::InvalidMessageType)
        ));
        assert!(matches!(
            context.remove_body::<Person>().await,
            Err(GetBodyError::InvalidMessageType)
        ));
    }
}
