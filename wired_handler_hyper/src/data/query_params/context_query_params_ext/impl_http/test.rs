use serde::{Deserialize, Serialize};
use wired_handler::StateSyncMutableInsert;

use crate::{
    prelude::*,
    state::{
        context::HttpRequestContext, global_state::GlobalState, request_state::RequestState,
        session_state::SessionState,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct TestParams {
    name: String,
    groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct TestParamsCopy {
    name: String,
    groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct TestParamsShouldFail {
    age: u8,
}

impl From<TestParams> for TestParamsCopy {
    fn from(value: TestParams) -> Self {
        Self {
            name: value.name,
            groups: value.groups,
        }
    }
}

impl From<TestParamsCopy> for TestParams {
    fn from(value: TestParamsCopy) -> Self {
        Self {
            name: value.name,
            groups: value.groups,
        }
    }
}

#[test]
fn test() {
    {
        let mut params = TestParams {
            name: "Test".to_string(),
            groups: vec!["group1".to_string(), "group2".to_string()],
        };
        let mut params_copy = TestParamsCopy::from(params.clone());
        let parsed_params = serde_html_form::to_string(params.clone()).unwrap();
        let parsed_params_copy = serde_html_form::to_string(params_copy.clone()).unwrap();

        assert_eq!(&parsed_params, &parsed_params_copy);

        let mut request_state = RequestState::default();
        request_state.insert(parsed_params.clone());
        let mut context = HttpRequestContext::from_states(
            GlobalState::default(),
            SessionState::default(),
            request_state,
        );

        assert_eq!(context.query_params::<TestParams>().unwrap(), Some(&params));
        assert_eq!(
            context.query_params::<TestParamsCopy>().unwrap(),
            Some(&params_copy)
        );
        assert_eq!(
            context.query_params_mut::<TestParams>().unwrap(),
            Some(&mut params)
        );
        assert_eq!(
            context.query_params_mut::<TestParamsCopy>().unwrap(),
            Some(&mut params_copy)
        );
        assert_eq!(
            context.remove_query_params::<TestParams>().unwrap(),
            Some(params.clone())
        );
        assert_eq!(
            context.remove_query_params::<TestParamsCopy>().unwrap(),
            Some(params_copy.clone())
        );

        assert!(context.query_params::<TestParamsShouldFail>().is_err());
        assert!(context.query_params_mut::<TestParamsShouldFail>().is_err());
        assert!(context
            .remove_query_params::<TestParamsShouldFail>()
            .is_err());
    }

    {
        let mut context = HttpRequestContext::from_states(
            GlobalState::default(),
            SessionState::default(),
            RequestState::default(),
        );
        assert!(context
            .query_params::<TestParamsShouldFail>()
            .unwrap()
            .is_none());
        assert!(context
            .query_params_mut::<TestParamsShouldFail>()
            .unwrap()
            .is_none());
        assert!(context
            .remove_query_params::<TestParamsShouldFail>()
            .unwrap()
            .is_none());
    }
}
