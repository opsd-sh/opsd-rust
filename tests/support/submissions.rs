//! Synthetic public API contracts for submission lifecycle operations.
use super::*;
use opsd::types::*;
use serde_json::{Value, json};
const ATTEMPT: &str = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";
const DATE: &str = "2026-10-01T12:00:00Z";

#[tokio::test]
async fn payroll_actions_preserve_versions_credentials_and_progress() {
    let mut pending: Value = serde_json::from_str(PAYROLL_RUN_DETAILS).unwrap();
    pending["status"] = json!("finalized");
    pending["version"] = json!(7);
    pending["submission"] = json!({"attempt_id":ATTEMPT,"attempt_count":2,"recorded_at":DATE,
        "initial_outcome_at":DATE,"business_outcome_at":null,"status":"pending"});
    pending["available_actions"] = json!([]);
    let history = json!({"payroll_run_id":pending["id"],"version":7,"current_revision_id":null,"current_attempt_id":ATTEMPT,"revisions":[]});
    let mut responses: Vec<_> = (0..5).map(|_| json(200, &pending.to_string())).collect();
    responses.push(json(200, &history.to_string()));
    let server = MockServer::start(responses);
    let client = OpsdClient::new_base(
        server.base_url.clone(),
        ApiCredential::new("test-token").unwrap(),
    )
    .unwrap();
    let business = "22222222-2222-4222-8222-222222222222".parse().unwrap();
    let run = "88888888-8888-4888-8888-888888888888".parse().unwrap();
    let retry = RetryPayrollRunRequest {
        expected_version: 6,
        sender_id: "sender".into(),
        password: HmrcPassword::new("secret"),
    };
    let result = client
        .retry_payroll_run(business, run, &retry)
        .await
        .unwrap();
    assert_eq!(serde_json::to_value(result).unwrap(), pending);
    client
        .resume_payroll_run(
            business,
            run,
            &ResumePayrollRunRequest::Polling {
                expected_version: 7,
            },
        )
        .await
        .unwrap();
    client
        .resume_payroll_run(
            business,
            run,
            &ResumePayrollRunRequest::Reconciliation {
                expected_version: 8,
                password: HmrcPassword::new("replacement"),
            },
        )
        .await
        .unwrap();
    client
        .reopen_payroll_run(
            business,
            run,
            &VersionedActionRequest {
                expected_version: 9,
            },
        )
        .await
        .unwrap();
    client
        .cancel_payroll_run(
            business,
            run,
            &VersionedActionRequest {
                expected_version: 10,
            },
        )
        .await
        .unwrap();
    assert_eq!(
        serde_json::to_value(client.get_payroll_run_history(business, run).await.unwrap()).unwrap(),
        history
    );
    let requests = server.finish();
    for (request, action) in requests
        .iter()
        .zip(["retry", "resume", "resume", "reopen", "cancel", "history"])
    {
        assert!(
            request
                .lines()
                .next()
                .unwrap()
                .contains(&format!("/{run}/{action} HTTP/1.1"))
        );
    }
    assert_json_body(
        &requests[0],
        json!({"expected_version":6,"sender_id":"sender","password":"secret"}),
    );
    assert_json_body(
        &requests[1],
        json!({"expected_version":7,"action":"polling"}),
    );
    assert_json_body(
        &requests[2],
        json!({"expected_version":8,"action":"reconciliation","password":"replacement"}),
    );
    assert_json_body(&requests[3], json!({"expected_version":9}));
    assert_json_body(&requests[4], json!({"expected_version":10}));
    assert!(!format!("{retry:?}").contains("secret"));
}

#[tokio::test]
async fn rollover_actions_and_reporting_use_public_routes() {
    let business: BusinessId = "22222222-2222-4222-8222-222222222222".parse().unwrap();
    let scheme: PayeSchemeId = "66666666-6666-4666-8666-666666666666".parse().unwrap();
    let rollover: PayeSchemeRolloverId = "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb".parse().unwrap();
    let practice: PracticeId = "cccccccc-cccc-4ccc-8ccc-cccccccccccc".parse().unwrap();
    let progress = json!({"id":rollover,"paye_scheme_id":scheme,"created_at":DATE,"version":3,"from_tax_year_start":2025,"tax_year_start":2026,"employment_allowance":"not_claiming","tax_code_notices":[],"preparation_failure":null,"status":"in_progress","progress":{"phase":"new_year_eps","final_fps":{"status":"accepted","attempt_id":ATTEMPT,"attempt_count":1},"eps":{"status":"credentials_required_to_send","message":"Supply credentials"}},"available_actions":[{"action":"send_eps"}]});
    let empty = json!({"attempts":[],"actions":[],"preparation_failures":[]});
    let history = json!({"rollover_id":rollover,"paye_scheme_id":scheme,"version":3,"current_attempt_id":null,"final_fps":empty,"new_year_eps":empty});
    let reporting = serde_json::to_value(reporting_details()).unwrap();
    let responses = vec![
        json(200, &json!([progress]).to_string()),
        json(200, &progress.to_string()),
        json(200, &history.to_string()),
        json(200, &progress.to_string()),
        json(200, &progress.to_string()),
        json(200, &progress.to_string()),
        json(200, &reporting.to_string()),
        json(200, &reporting.to_string()),
    ];
    let server = MockServer::start(responses);
    let client = OpsdClient::new_base(
        server.base_url.clone(),
        ApiCredential::new("test-token").unwrap(),
    )
    .unwrap();
    assert_eq!(
        client
            .list_paye_scheme_rollovers(business, scheme)
            .await
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        serde_json::to_value(
            client
                .get_paye_scheme_rollover(business, scheme, rollover)
                .await
                .unwrap()
        )
        .unwrap(),
        progress
    );
    assert_eq!(
        serde_json::to_value(
            client
                .get_paye_scheme_rollover_history(business, scheme, rollover)
                .await
                .unwrap()
        )
        .unwrap(),
        history
    );
    client
        .retry_paye_scheme_rollover(
            business,
            scheme,
            rollover,
            &RetryRolloverRequest {
                expected_version: 3,
                sender_id: "sender".into(),
                password: HmrcPassword::new("secret"),
            },
        )
        .await
        .unwrap();
    client
        .resume_paye_scheme_rollover(
            business,
            scheme,
            rollover,
            &ResumeRolloverRequest::SendEps {
                expected_version: 4,
                sender_id: "sender".into(),
                password: HmrcPassword::new("secret"),
            },
        )
        .await
        .unwrap();
    client
        .cancel_paye_scheme_rollover(
            business,
            scheme,
            rollover,
            &VersionedActionRequest {
                expected_version: 5,
            },
        )
        .await
        .unwrap();
    client.get_practice_hmrc_reporting(practice).await.unwrap();
    client
        .update_practice_hmrc_reporting(practice, &reporting_details())
        .await
        .unwrap();
    let requests = server.finish();
    let base = format!("/v1/businesses/{business}/paye-schemes/{scheme}/rollovers");
    for (i, (request, (method, suffix))) in requests
        .iter()
        .zip([
            ("GET", ""),
            ("GET", ""),
            ("GET", "history"),
            ("POST", "retry"),
            ("POST", "resume"),
            ("POST", "cancel"),
        ])
        .enumerate()
    {
        let path = if i == 0 {
            base.clone()
        } else if suffix.is_empty() {
            format!("{base}/{rollover}")
        } else {
            format!("{base}/{rollover}/{suffix}")
        };
        assert_eq!(
            request.lines().next(),
            Some(format!("{method} {path} HTTP/1.1").as_str())
        );
    }
    assert_json_body(
        &requests[4],
        json!({"action":"send_eps","expected_version":4,"sender_id":"sender","password":"secret"}),
    );
    assert!(requests[6].starts_with(&format!("GET /v1/practices/{practice}/hmrc-reporting ")));
    assert!(requests[7].starts_with(&format!("PUT /v1/practices/{practice}/hmrc-reporting ")));
    assert_json_body(&requests[7], reporting);
}

#[test]
fn nullable_updates_and_history_are_lossless() {
    for value in [
        json!({}),
        json!({"title":null,"passport_number":null,"national_insurance_number":null}),
        json!({"title":"Dr","passport_number":"EXAMPLE","national_insurance_number":"AB123456C"}),
    ] {
        let parsed: UpdateEmployeeRequest = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(parsed).unwrap(), value);
    }
    for status in [
        json!({"status":"sending"}),
        json!({"status":"pending"}),
        json!({"status":"reconciling"}),
        json!({"status":"not_sent","message":"Not sent"}),
        json!({"status":"credentials_required","message":"Credentials required"}),
        json!({"status":"polling_paused","message":"Paused"}),
        json!({"status":"reconciliation_paused","message":"Paused"}),
        json!({"status":"needs_attention","message":"Contact support"}),
        json!({"status":"rejected","reason":{"code":"authentication_failed","message":"Check credentials"}}),
        json!({"status":"accepted"}),
    ] {
        let mut value = json!({"id":ATTEMPT,"attempt_number":1,"recorded_at":DATE,"transaction_id":"test-transaction","environment":"test","initial_outcome":{"outcome":"acknowledged","recorded_at":DATE},"business_outcome":{"outcome":"accepted","recorded_at":DATE},"cleanup":{"status":"pending","next_action_at":DATE}});
        value
            .as_object_mut()
            .unwrap()
            .extend(status.as_object().unwrap().clone());
        let parsed: SubmissionAttempt = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(parsed).unwrap(), value);
    }
}
