use candid::Principal;
use ic_cdk::{
    api::{
        self,
        call::{self, RejectionCode},
    },
    storage,
};

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn whoami() -> Principal {
    api::caller()
}

ic_cdk::export_candid!();

mod test {
    #[test]
    fn test_greet() {
        let name = "test".to_string();
        let result = super::greet(name);
        assert_eq!(result, "Hello, test!");
    }
}
