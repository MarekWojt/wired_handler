use hyper::body::Bytes;
use serde::{Deserialize, Serialize};

use crate::{
    data::request_body::GetBodyError,
    prelude::*,
    state::{
        context::HttpRequestContext, global_state::GlobalState, request_state::RequestState,
        session_state::SessionState,
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

    {
        let person = Person {
            name: "Franz".to_string(),
            age: 81,
        };
        let mut request_state = RequestState::default();
        request_state.insert(Bytes::from(serde_json::to_vec(&person).unwrap()));

        let mut context = HttpRequestContext::from_states(
            global_state.clone(),
            session_state.clone(),
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
}
