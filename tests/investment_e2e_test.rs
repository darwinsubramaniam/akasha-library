pub(crate) mod mocks;
use akasha::{money_investment::{service::InvestmentService, Investment, InvestmentType}, fiat::{service::fixer_api::FixerApiService, Fiat}};
use chrono::Utc;
use mocks::mock_investmentdb::MockInvestmentDBService;
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
    let _ = test_await!(investment_service.add_investment(&new_investment));
    let result = test_await!(investment_service.get_investments_by_currency(&"DAR".to_string()));
    let all = test_await!(investment_service.get_investments()).unwrap();
    dbg!(all);
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
    let result = test_await!(investment_service.get_investments());
    let result = result.unwrap();
    assert_eq!(result.len(), 3);
}

#[test]
fn test_investment_by_currency() {
    let mut mock_investment_db_service = MockInvestmentDBService::default();
    let investment_service = InvestmentService::new(&mut mock_investment_db_service);
    let result = test_await!(investment_service.get_investments_by_currency(&"SGD".to_string()));
    let result = result.unwrap();
    assert_eq!(result.len(), 2);

    let result = test_await!(investment_service.get_investments_by_currency(&"MYR".to_string()));
    let result = result.unwrap();
    assert_eq!(result.len(), 1);
}

#[test]
fn tes_service_filter_by_type() {
    let mut mock_investment_db_service = MockInvestmentDBService::default();
    let investment_service = InvestmentService::new(&mut mock_investment_db_service);

    let result = test_await!(investment_service.get_investments_by_type(&InvestmentType::DEPOSIT));
    let result = result.unwrap();
    assert_eq!(result.len(), 2);

    let result = test_await!(investment_service.get_investments_by_type(&InvestmentType::WITHDRAW));
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
    let _ =
        test_await!(investment_service.update_investment_by_id(&"1".to_string(), &new_investment));

    let result = test_await!(investment_service.get_investment_by_id(id)).unwrap().unwrap();
    assert_eq!(result.amount(), &200.0);
    assert_eq!(result.currency(), &"DAR".to_string());
    assert_eq!(result.investment_type(), &InvestmentType::DEPOSIT);
}

#[test]
fn test_delete_by_id(){
    let mut mock_investment_db_service = MockInvestmentDBService::default();
    let mut investment_service = InvestmentService::new(&mut mock_investment_db_service);
    let id = "1";
    let _ = test_await!(investment_service.delete_investment_by_id(id));
    let result = test_await!(investment_service.get_investment_by_id(id)).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_total_investment_by_currency(){
    let mut mock_investment_db_service = MockInvestmentDBService::default();
    let investment_service = InvestmentService::new(&mut mock_investment_db_service);
    let result = test_await!(investment_service.get_total_investment_by_currency(&"SGD".to_string()));
    let result = result.unwrap();
    assert_eq!(result, 90.0);
}

#[test]
#[ignore = "Require to make API_CALL to 3rd party service. Requires API_KEY"]
fn test_total_investment_by_type(){
    let mut mock_investment_db_service = MockInvestmentDBService::default();
    let investment_service = InvestmentService::new(&mut mock_investment_db_service);
    let api_key = std::env::var("FIXER_API_KEY").expect("FIXER_API_KEY is not set");
    let service = FixerApiService::new(&api_key);
    let desired_fiat = Fiat::new("Singapore Dollar".to_string(), "SGD".to_string());

    let result = test_await!(
        investment_service.get_total_investment_by_type(&InvestmentType::DEPOSIT,&desired_fiat, &service)
    ).unwrap();
    assert!(result > 0.0);

    let result = test_await!(
        investment_service.get_total_investment_by_type(&InvestmentType::WITHDRAW,&desired_fiat, &service)
    ).unwrap();
    assert!(result > 0.0);
}

#[test]
#[ignore = "Require to make API_CALL to 3rd party service. Requires API_KEY"]
fn test_project_total_investment(){
    let mut mock_investment_db_service = MockInvestmentDBService::default();
    let investment_service = InvestmentService::new(&mut mock_investment_db_service);
    let api_key = std::env::var("FIXER_API_KEY").expect("FIXER_API_KEY is not set");
    let service = FixerApiService::new(&api_key);
    let desired_fiat = Fiat::new("Singapore Dollar".to_string(), "SGD".to_string());
    let result = test_await!(investment_service.project_total_investment(&desired_fiat, &service));
    let result = result.unwrap();
    assert!(result > 0.0);
}