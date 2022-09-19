use std::env;

use akasha::fiat::{service::fixer_api::FixerApiService, FiatService, Fiat};

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
    let currencies = test_await!(service.get_all_supported_fiat());
    assert!(currencies.unwrap().len() > 0);
}

#[test]
fn test_conversion(){
    let usd_fiat:Fiat = Fiat::new("United States Dollar".to_string(), "USD".to_string());
    let jpy_fiat:Fiat = Fiat::new("Japanese Yen".to_string(), "JPY".to_string());
    let api_key = env::var("FIXER_API_KEY").expect("FIXER_API_KEY is not set");
    let service = FixerApiService::new(&api_key);
    let converted_amount_to_jpy = test_await!(usd_fiat.conversion(100.0, &jpy_fiat, &service));
    let amount = converted_amount_to_jpy.unwrap();
    println!("100 USD to JPY is : {}", amount); 
    assert!(amount > 0.0);
}