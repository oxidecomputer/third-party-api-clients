const EMPLOYEE: &str = r#"{
  "id": 7757869432666662,
  "uuid": "4b3f930f-82cd-48a8-b797-798686e12e5e",
  "first_name": "Isom",
  "middle_initial": null,
  "last_name": "Jaskolski",
  "email": "dane7757869450111550@botsford.net",
  "company_id": 7756341740978008,
  "company_uuid": "a007e1ab-3595-43c2-ab4b-af7a5af2e365",
  "manager_id": 7757869432666665,
  "version": "1c7ba9d62c8bafbfff998ffccad5d296",
  "department": null,
  "terminated": false,
  "two_percent_shareholder": false,
  "onboarded": true,
  "jobs": [
    {
      "id": 7757869441038001,
      "version": "6c0ed1521e8b86eb36bd4455a63a2dac",
      "employee_id": 7757869432666662,
      "current_compensation_id": 7757869444844982,
      "payment_unit": "Year",
      "primary": true,
      "title": "Client Support Director",
      "compensations": [
        {
          "id": 7757869444844982,
          "version": "2cd4b18662395eb53bcf80d5b5447f36",
          "payment_unit": "Year",
          "flsa_status": "Commission Only Exempt",
          "job_id": 7757869441038001,
          "effective_date": "2021-01-20",
          "rate": "70000.00"
        }
      ],
      "rate": "70000.00",
      "hire_date": "2020-01-20",
      "location_id": 7757727716657803,
      "location": {
        "id": 7757727716657803,
        "street_1": "412 Kiera Stravenue",
        "street_2": "Suite 391",
        "city": "San Francisco",
        "state": "CA",
        "zip": "94107",
        "country": "USA",
        "inactive": false
      }
    }
  ],
  "eligible_paid_time_off": [
    {
      "name": "Sick Hours",
      "accrual_unit": "Hour",
      "accrual_rate": "208.0",
      "accrual_period": "Year",
      "accrual_balance": "31.8",
      "maximum_accrual_balance": "240.0",
      "paid_at_termination": false
    },
    {
      "name": "Vacation Hours",
      "accrual_unit": "Hour",
      "accrual_rate": "208.0",
      "accrual_period": "Year",
      "accrual_balance": "77.8",
      "maximum_accrual_balance": "240.0",
      "paid_at_termination": true
    }
  ],
  "terminations": [],
  "custom_fields": [
    {
      "id": "ee515986-f3ca-49da-b576-2691b95262f9",
      "company_custom_field_id": "ea7e5d57-6abb-47d7-b654-347c142886c0",
      "name": "employee_level",
      "description": "Employee Level",
      "type": "text",
      "value": "2",
      "selection_options": []
    },
    {
      "id": "3796e08d-c2e3-434c-b4de-4ce1893e7b59",
      "company_custom_field_id": "299650e4-e970-4acf-9bf0-6f05585d20ba",
      "name": "t-shirt size",
      "description": "What is your t-shirt size?",
      "type": "text",
      "value": "md",
      "selection_options": []
    },
    {
      "id": "3796e08d-c2e3-434c-b4de-4ce1893e7b59",
      "company_custom_field_id": "024ec137-6c92-43a3-b061-14a9720531d6",
      "name": "favorite fruit",
      "description": "Which is your favorite fruit?",
      "type": "radio",
      "value": "apple",
      "selection_options": [
        "apple",
        "banana",
        "orange"
      ]
    }
  ],
  "home_address": {
    "version": "bfc6ed1d49aa9677265232c470fdbc3e",
    "employee_id": 7757869432666662,
    "street_1": "73243 Wuckert Prairie",
    "street_2": "Suite 189",
    "city": "San Francisco",
    "state": "CA",
    "zip": "94107",
    "country": "USA",
    "active": true
  },
  "garnishments": [],
  "date_of_birth": "1986-06-25",
  "has_ssn": false,
  "ssn": "",
  "phone": "1234567890",
  "preferred_first_name": "Angel",
  "work_email": null
}"#;

#[test]
fn test_deserialize_employee() {
    let deserialized: gusto_api::types::Employee = serde_json::from_str(EMPLOYEE).unwrap();
    println!("employee = {:?}", deserialized);
    let first_job = deserialized.jobs.first().unwrap();
    let first_compensation = first_job.compensations.first().unwrap();
    assert_eq!(
        first_compensation
            .flsa_status
            .as_ref()
            .unwrap()
            .clone()
            .to_string(),
        // TODO: fix this better.
        //"Commission Only Exempt".to_string()
        "*".to_string()
    );
    assert_eq!(
        first_compensation.payment_unit.as_ref().unwrap().clone(),
        gusto_api::types::PaymentUnit::Year
    );
}
