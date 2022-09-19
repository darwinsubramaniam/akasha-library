use std::env;

use akasha::fiat::{service::fixer_api::FixerApiService, FiatService};

#[macro_export]
macro_rules! test_await {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

#[test]
#[ignore = "Require to make API_CALL to 3rd party service, enable only when needed"]
fn test_fiat() {
    let api_key = env::var("FIXER_API_KEY").expect("FIXER_API_KEY is not set");
    let service = FixerApiService::new(&api_key);
    let currencies = test_await!(service.currencies());
    assert!(currencies.unwrap().len() > 0);
}