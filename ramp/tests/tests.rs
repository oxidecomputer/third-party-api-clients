const USER: &str = r#"{
  "business_id": "00316d69-a36f-4a24-883a-a0a25353686c",
  "department_id": "7e2617f5-0892-4658-8d76-86d500595eeb",
  "email": "thing@example.com",
  "first_name": "Kevin",
  "id": "9b84d870-f348-43d6-baa4-77181d3cc0f9",
  "last_name": "Thing",
  "location_id": null,
  "manager_id": "df642bf5-784f-4350-b60f-9a37347b3b79",
  "phone": "+15551234321",
  "role": "BUSINESS_USER"
}"#;

const REIMBURSEMENT: &str = r#"{
  "amount": 301.96,
  "created_at": "2021-08-09T15:58:35.255694",
  "currency": "USD",
  "id": "c9a1c47d-e785-43c7-ac67-47b1979e0d2b",
  "merchant": "Various",
  "receipts": [
    "2fae6f28-8872-4f9b-bae8-7f588125d72a"
  ],
  "transaction_date": "2021-08-05",
  "user_id": "f518c870-8c59-418e-a6db-bb27787deeb5"
}"#;

#[test]
fn test_deserialize() {
    let deserialized: ramp_api::types::User = serde_json::from_str(USER).unwrap();
    println!("user = {:?}", deserialized);
    assert_eq!(deserialized.role, ramp_api::types::Role::BusinessUser);

    let deserialized: ramp_api::types::Reimbursement = serde_json::from_str(REIMBURSEMENT).unwrap();
    println!("reimbursement = {:?}", deserialized);

    let t = ramp_api::types::PostUsersDeferredRequest {
        department_id: "".to_string(),
        direct_manager_id: "".to_string(),
        email: "3".to_string(),
        first_name: "Joe".to_string(),
        last_name: "Smith".to_string(),
        location_id: "".to_string(),
        phone: "5553245132".to_string(),
        role: ramp_api::types::Role::BusinessUser,
    };
    let s = serde_json::to_string_pretty(&t).unwrap();
    println!("string_user_request = {}", s);
    assert!(true, "{}", s.contains("BUSINESS_USER"));
}
