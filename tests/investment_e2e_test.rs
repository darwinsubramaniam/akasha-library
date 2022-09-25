pub(crate) mod mocks;
use akasha::{
    fiat::Fiat,
    money_investment::{service::InvestmentService, Investment, InvestmentType},
};
use chrono::Utc;
use mocks::mock_investmentdb::MockInvestmentDBService;

use crate::mocks::mock_fiat_service::MockFiatService;
#[macro_export]
macro_rules! test_await {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

#[test]
fn test_investment_service() {
    let mut mock_investment_db_service = MockInvestmentDBService::default();
    let mut investment_service = InvestmentService::new(&mut mock_investment_db_service);
    let new_investment = Investment::new(
        Some(uuid::Uuid::new_v4().to_string()),
        InvestmentType::DEPOSIT,
        200.0,
        "DAR".to_string(),
        Utc::now(),
    );
    let _ = test_await!(investment_service.write_db().create(&new_investment));
    let result = test_await!(investment_service
        .read_db()
        .get_by_currency(&"DAR".to_string()));

    let result = result.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].amount(), &200.0);
    assert_eq!(result[0].currency(), &"DAR".to_string());
    assert_eq!(result[0].investment_type(), &InvestmentType::DEPOSIT);
}

#[test]
fn test_investment_service_get_investments() {
    let mut mock_investment_db_service = MockInvestmentDBService::default();
    let investment_service = InvestmentService::new(&mut mock_investment_db_service);
    let result = test_await!(investment_service.read_db().get());
    let result = result.unwrap();
    assert_eq!(result.len(), 3);
}

#[test]
fn test_investment_by_currency() {
    let mut mock_investment_db_service = MockInvestmentDBService::default();
    let investment_service = InvestmentService::new(&mut mock_investment_db_service);
    let result = test_await!(investment_service
        .read_db()
        .get_by_currency(&"SGD".to_string()));
    let result = result.unwrap();
    assert_eq!(result.len(), 2);

    let result = test_await!(investment_service
        .read_db()
        .get_by_currency(&"MYR".to_string()));
    let result = result.unwrap();
    assert_eq!(result.len(), 1);
}

#[test]
fn tes_service_filter_by_type() {
    let mut mock_investment_db_service = MockInvestmentDBService::default();
    let investment_service = InvestmentService::new(&mut mock_investment_db_service);

    let result = test_await!(investment_service
        .read_db()
        .get_by_type(&InvestmentType::DEPOSIT));
    let result = result.unwrap();
    assert_eq!(result.len(), 2);

    let result = test_await!(investment_service
        .read_db()
        .get_by_type(&InvestmentType::WITHDRAW));
    let result = result.unwrap();
    assert_eq!(result.len(), 1);
}

#[test]
fn test_update_by_id() {
    let mut mock_investment_db_service = MockInvestmentDBService::default();
    let mut investment_service = InvestmentService::new(&mut mock_investment_db_service);
    let id = "1";
    let new_investment = Investment::new(
        Some(id.to_string()),
        InvestmentType::DEPOSIT,
        200.0,
        "DAR".to_string(),
        Utc::now(),
    );
    let _ = test_await!(investment_service.write_db().update_by_id(&"1".to_string(), &new_investment));

    let result = test_await!(investment_service.read_db().get_by_id(id))
        .unwrap()
        .unwrap();
    assert_eq!(result.amount(), &200.0);
    assert_eq!(result.currency(), &"DAR".to_string());
    assert_eq!(result.investment_type(), &InvestmentType::DEPOSIT);
}

#[test]
fn test_delete_by_id() {
    let mut mock_investment_db_service = MockInvestmentDBService::default();
    let mut investment_service = InvestmentService::new(&mut mock_investment_db_service);
    let id = "1";
    let _ = test_await!(investment_service.write_db().delete_by_id(id));
    let result = test_await!(investment_service.read_db().get_by_id(id)).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_total_investment_by_currency() {
    let mut mock_investment_db_service = MockInvestmentDBService::default();
    let investment_service = InvestmentService::new(&mut mock_investment_db_service);
    let result = test_await!(investment_service.total_by_currency(&"SGD".to_string()));
    let result = result.unwrap();
    assert_eq!(result, 90.0);
}

#[test]
fn test_total_investment_by_type() {
    let mut mock_investment_db_service = MockInvestmentDBService::default();
    let investment_service = InvestmentService::new(&mut mock_investment_db_service);
    let service = MockFiatService::default();
    let desired_fiat = Fiat::new("Singapore Dollar".to_string(), "SGD".to_string());

    let result = test_await!(investment_service.total_by_type(
        &InvestmentType::DEPOSIT,
        &desired_fiat,
        &service
    ))
    .unwrap();
    assert!(result == (100.0 + (10.0 / 2.0)));

    let result = test_await!(investment_service.total_by_type(
        &InvestmentType::WITHDRAW,
        &desired_fiat,
        &service
    ))
    .unwrap();
    assert!(result == 10.0);
}

#[test]
fn test_project_total_investment() {
    let mut mock_investment_db_service = MockInvestmentDBService::default();
    let investment_service = InvestmentService::new(&mut mock_investment_db_service);
    let service = MockFiatService::default();
    let desired_fiat = Fiat::new("Singapore Dollar".to_string(), "SGD".to_string());

    let result = test_await!(investment_service.total(&desired_fiat, &service));
    let result = result.unwrap();
    assert!(result == (100.0 - 10.0 + (10.0 / 2.0)));
}
