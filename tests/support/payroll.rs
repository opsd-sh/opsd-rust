use super::*;
use opsd::types::*;
use serde_json::json;

fn calculation() -> serde_json::Value {
    let amounts = json!({"gross_pay_pence":300000,"income_tax_pence":0,"employee_ni_pence":0,"employer_ni_pence":0,"net_pay_pence":300000});
    let zero = json!({"gross_pay_pence":0,"income_tax_pence":0,"employee_ni_pence":0,"employer_ni_pence":0,"net_pay_pence":0});
    let mut payment = amounts.clone();
    payment["employment_id"] = json!("77777777-7777-4777-8777-777777777777");
    payment["ytd"] = amounts.clone();
    let bands = json!({"at_lel_pence":0,"above_lel_to_pt_pence":0,"above_pt_to_uel_pence":0});
    payment["ni_earnings"] = bands.clone();
    payment["ni_earnings_ytd"] = bands.clone();
    payment["inputs"] = json!({"tax_code":"NT","tax_basis":"non_cumulative","ni_category":"X","previous_employment_taxable_pay_pence":0,"previous_employment_tax_paid_pence":0,"previous_ytd":zero});
    payment["inputs"]["previous_ni_earnings_ytd"] = bands;
    json!({"payroll_run_id":"88888888-8888-4888-8888-888888888888","payment_date":"2026-09-30","tax_year_start":2026,"tax_month":6,"payments":[payment],"totals":amounts})
}

#[tokio::test]
async fn payroll_client_preserves_settings_calculations_and_routes() {
    let schedule = json!({"id":"99999999-9999-4999-8999-999999999999","paye_scheme_id":"66666666-6666-4666-8666-666666666666","name":"Monthly staff","frequency":"monthly","created_at":"2026-09-04T12:00:00Z"});
    let preview = calculation();
    let mut saved: serde_json::Value = serde_json::from_str(PAYROLL_RUN_DETAILS).unwrap();
    saved["calculation"] = preview.clone();
    let rollover = json!({"id":"aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa","paye_scheme_id":"66666666-6666-4666-8666-666666666666","version":1,"created_at":"2026-09-04T12:00:00Z","from_tax_year_start":2025,"tax_year_start":2026,"employment_allowance":"not_claiming","status":"in_progress","progress":{"phase":"final_fps","final_fps":{"attempt_id":"bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb","attempt_count":1,"status":"pending","message":null}},"available_actions":[],"preparation_failure":null,"tax_code_notices":[{"employment_id":"77777777-7777-4777-8777-777777777777","tax_code":"1257L","tax_basis":"cumulative"}]});
    let server = MockServer::start(vec![
        json_response(201, &schedule),
        json_response(200, &json!([schedule])),
        json(200, EMPLOYMENT),
        json(200, EMPLOYMENT),
        json(200, EMPLOYMENT),
        json_response(200, &preview),
        json_response(200, &saved),
        json_response(200, &saved),
        no_content(),
        no_content(),
        json_response(200, &rollover),
    ]);
    let client = OpsdClient::new_base(
        server.base_url.clone(),
        ApiCredential::new("test-token").unwrap(),
    )
    .unwrap();
    let business = BusinessId::parse("22222222-2222-4222-8222-222222222222").unwrap();
    let scheme = PayeSchemeId::parse("66666666-6666-4666-8666-666666666666").unwrap();
    let employment = EmploymentId::parse("77777777-7777-4777-8777-777777777777").unwrap();
    let run = PayrollRunId::parse("88888888-8888-4888-8888-888888888888").unwrap();
    let created = client
        .create_pay_schedule(
            business,
            &CreatePayScheduleRequest {
                paye_scheme_id: scheme,
                name: PayScheduleName::parse("Monthly staff").unwrap(),
                frequency: PayFrequency::Monthly,
            },
        )
        .await
        .unwrap();
    assert_eq!(serde_json::to_value(&created).unwrap(), schedule);
    assert_eq!(
        client
            .list_pay_schedules(business)
            .await
            .unwrap()
            .schedules
            .len(),
        1
    );
    let read = client.get_employment(business, employment).await.unwrap();
    assert_eq!(
        read.employment.payroll_settings.tax_code.as_deref(),
        Some("1257L")
    );
    assert_eq!(
        read.employment
            .payroll_settings
            .monthly_gross_pay_pence
            .unwrap()
            .pence(),
        300000
    );
    client
        .update_employment(
            business,
            employment,
            &UpdateEmploymentRequest::Settings(UpdateEmploymentSettings {
                monthly_gross_pay_pence: Some(NonNegativePence::new(310000).unwrap()),
                ..Default::default()
            }),
        )
        .await
        .unwrap();
    client
        .update_employment(
            business,
            employment,
            &UpdateEmploymentRequest::TaxCodeNotice(ApplyEmploymentTaxCodeNotice {
                tax_code_notice: TaxCodeNotice::Cumulative {
                    tax_year_start: 2026,
                    tax_code: "1257L".into(),
                    previous_employment: Some(PreviousEmploymentAmounts {
                        taxable_pay_pence: NonNegativePence::new(100000).unwrap(),
                        tax_paid_pence: -200,
                    }),
                },
                monthly_gross_pay_pence: None,
                ni_category: None,
                normal_weekly_hours_band: None,
            }),
        )
        .await
        .unwrap();
    assert_eq!(
        serde_json::to_value(client.preview_payroll_run(business, run).await.unwrap()).unwrap(),
        preview
    );
    assert_eq!(
        serde_json::to_value(client.get_payroll_run(business, run).await.unwrap()).unwrap(),
        saved
    );
    assert_eq!(
        serde_json::to_value(
            client
                .finalize_payroll_run(business, run, &finalize_request())
                .await
                .unwrap()
        )
        .unwrap(),
        saved
    );
    client
        .update_payroll_payment(
            business,
            run,
            employment,
            &UpdatePayrollPaymentRequest::GrossPay {
                normal_weekly_hours_band: None,
                gross_pay_pence: NonNegativePence::new(150000).unwrap(),
            },
        )
        .await
        .unwrap();
    client.discard_payroll_run(business, run).await.unwrap();
    let request = RolloverPayeSchemeRequest {
        expected_generation: 0,
        sender_id: "test-sender".into(),
        password: HmrcPassword::new("test-password"),
        tax_year_start: 2026,
        employment_allowance: EmploymentAllowance::NotClaiming,
        tax_code_notices: vec![RolloverTaxCodeNotice {
            employment_id: employment,
            tax_code: "1257L".into(),
            tax_basis: TaxBasis::Cumulative,
        }],
    };
    assert_eq!(
        serde_json::to_value(
            client
                .rollover_paye_scheme(business, scheme, &request)
                .await
                .unwrap()
        )
        .unwrap(),
        rollover
    );
    let requests = server.finish();
    let prefix = format!("/v1/businesses/{business}");
    for (actual, (method, path)) in requests.iter().zip([
        ("POST", "pay-schedules".to_owned()),
        ("GET", "pay-schedules".to_owned()),
        ("GET", format!("employments/{employment}")),
        ("PATCH", format!("employments/{employment}")),
        ("PATCH", format!("employments/{employment}")),
        ("GET", format!("payroll-runs/{run}/preview")),
        ("GET", format!("payroll-runs/{run}")),
        ("POST", format!("payroll-runs/{run}/finalize")),
        (
            "PATCH",
            format!("payroll-runs/{run}/employments/{employment}"),
        ),
        ("DELETE", format!("payroll-runs/{run}")),
        ("POST", format!("paye-schemes/{scheme}/rollover")),
    ]) {
        assert_eq!(
            actual.lines().next(),
            Some(format!("{method} {prefix}/{path} HTTP/1.1").as_str())
        );
        assert!(actual.contains("authorization: Bearer test-token\r\n"));
    }
    assert_json_body(&requests[3], json!({"monthly_gross_pay_pence":310000}));
    assert_json_body(
        &requests[4],
        json!({"tax_code_notice":{"tax_year_start":2026,"tax_code":"1257L","tax_basis":"cumulative","previous_employment":{"taxable_pay_pence":100000,"tax_paid_pence":-200}}}),
    );
    assert_json_body(&requests[8], json!({"gross_pay_pence":150000}));
    assert_json_body(&requests[10], serde_json::to_value(request).unwrap());
}
fn json_response(status: u16, value: &serde_json::Value) -> MockResponse {
    json(status, &value.to_string())
}

#[test]
fn starter_and_notice_inputs_encode_only_the_selected_route() {
    let p45 = StarterInformation::P45 {
        p45: P45 {
            leaving_date: "2026-08-31".parse().unwrap(),
            tax_code: "1257L".into(),
            tax_details: P45TaxDetails::Cumulative {
                total_pay_to_date_pence: NonNegativePence::new(100000).unwrap(),
                total_tax_to_date_pence: -20,
            },
        },
        starter_declaration: Some(StarterDeclaration::B),
    };
    let wire = serde_json::to_value(&p45).unwrap();
    assert_eq!(
        wire,
        json!({"type":"p45","p45":{"leaving_date":"2026-08-31","tax_code":"1257L","tax_details":{"basis":"cumulative","total_pay_to_date_pence":100000,"total_tax_to_date_pence":-20}},"starter_declaration":"B"})
    );
    assert_eq!(
        serde_json::from_value::<StarterInformation>(wire).unwrap(),
        p45
    );
    let notice = UpdateEmploymentRequest::TaxCodeNotice(ApplyEmploymentTaxCodeNotice {
        tax_code_notice: TaxCodeNotice::NonCumulative {
            tax_year_start: 2026,
            tax_code: "BR".into(),
        },
        monthly_gross_pay_pence: None,
        ni_category: None,
        normal_weekly_hours_band: None,
    });
    assert_eq!(
        serde_json::to_value(notice).unwrap(),
        json!({"tax_code_notice":{"tax_year_start":2026,"tax_code":"BR","tax_basis":"non_cumulative"}})
    );
    assert!(serde_json::from_value::<UpdateEmploymentRequest>(json!({"start_date":"2026-10-01","tax_code_notice":{"tax_year_start":2026,"tax_code":"BR","tax_basis":"non_cumulative"}})).is_err());
    assert!(serde_json::from_value::<TaxCodeNotice>(json!({"tax_year_start":2026,"tax_code":"BR","tax_basis":"non_cumulative","previous_employment":{"taxable_pay_pence":0,"tax_paid_pence":0}})).is_err());
    assert!(NonNegativePence::new(-1).is_err());
    assert!(EmploymentDate::parse("0000-01-01").is_err());
    assert!(EmploymentDate::parse("2026-02-29").is_err());
}
