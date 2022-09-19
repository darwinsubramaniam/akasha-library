use akasha::crypto::service::{coingecko::CoingeckoService, CryptoService};

#[macro_export]
macro_rules! test_await {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

#[test]
#[ignore = "Long Running Test"]
fn test_able_fetch_crypto_basic() {
    let service = CoingeckoService::new();
    let cryptos = test_await!(service.cryptos());
    assert!(cryptos.unwrap().len() > 0);
}

#[test]
#[ignore = "Long Running Test"]
fn test_able_fetch_crypto_image() {
    let service = CoingeckoService::new();
    let image = test_await!(service.image("bitcoin"));
    assert!(image.unwrap().small.is_empty() == false);
}

#[test]
fn test_able_to_fetch_support_currencies() {
    let service = CoingeckoService::new();
    let currencies = test_await!(service.get_supported_quoted_currency());
    assert!(currencies.unwrap().len() > 0);
}

#[test]
fn test_conversion_able_to_be_performed() {
    let service = CoingeckoService::new();
    let result = test_await!(service.conversion(100.0, "bitcoin", "usd"));
    let amount = result.unwrap();
    dbg!(amount);
    assert!(!amount.is_nan());
}
