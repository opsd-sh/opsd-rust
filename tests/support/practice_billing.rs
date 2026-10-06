use super::{MockResponse, MockServer, json, no_content};
use opsd::{ApiCredential, Error, OpsdClient, types::*};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json as value};

const PRACTICE: &str = "11111111-1111-4111-8111-111111111111";
const BUSINESS: &str = "22222222-2222-4222-8222-222222222222";
const USER: &str = "33333333-3333-4333-8333-333333333333";
const INVITATION: &str = "44444444-4444-4444-8444-444444444444";
const INVOICE: &str = "55555555-5555-4555-8555-555555555555";
const DATE: &str = "2026-10-01T12:00:00Z";

fn client(server: &MockServer) -> OpsdClient {
    OpsdClient::new_base(
        server.base_url.clone(),
        ApiCredential::new("test-token").unwrap(),
    )
    .unwrap()
}

fn assert_request(request: &str, method: &str, path: &str, body: Option<Value>) {
    assert_eq!(
        request.lines().next().unwrap(),
        format!("{method} /v1/{path} HTTP/1.1")
    );
    let headers = request
        .split("\r\n\r\n")
        .next()
        .unwrap()
        .to_ascii_lowercase();
    assert!(headers.contains("authorization: bearer test-token\r\n"));
    assert!(headers.contains("accept: application/"));
    let actual = request.split_once("\r\n\r\n").unwrap().1;
    if let Some(body) = body {
        assert_eq!(serde_json::from_str::<Value>(actual).unwrap(), body);
    } else {
        assert!(actual.is_empty());
    }
}

#[tokio::test]
async fn practice_lifecycle_uses_typed_models_and_correct_routes() {
    let practice = value!({"id":PRACTICE,"name":"Accountants","role":"admin"});
    let business = value!({"id":BUSINESS,"name":"Client","practice_id":PRACTICE});
    let member =
        value!({"id":USER,"email":"person@example.com","email_verified":true,"role":"member"});
    let invitation = value!({"id":INVITATION,"email":"person@example.com","role":"member","status":"pending","expires_at":DATE});
    let pending = value!({"id":INVITATION,"practice_id":PRACTICE,"practice_name":"Accountants","invited_by_email":"admin@example.com","role":"member","expires_at":DATE});
    let server = MockServer::start(vec![
        json(201, &practice.to_string()),
        json(200, &value!([practice]).to_string()),
        json(200, &practice.to_string()),
        json(201, &business.to_string()),
        json(200, &value!([business]).to_string()),
        json(200, &value!([member]).to_string()),
        no_content(),
        no_content(),
        json(201, &invitation.to_string()),
        json(200, &value!([invitation]).to_string()),
        no_content(),
        json(200, &value!([pending]).to_string()),
        json(200, &practice.to_string()),
        no_content(),
    ]);
    let client = client(&server);
    let pid = PracticeId::parse(PRACTICE).unwrap();
    let uid = UserId::parse(USER).unwrap();
    let iid = PracticeInvitationId::parse(INVITATION).unwrap();
    assert_eq!(
        client
            .create_practice(&CreatePracticeRequest {
                name: PracticeName::parse("Accountants").unwrap()
            })
            .await
            .unwrap()
            .membership
            .role,
        PracticeRole::Admin
    );
    assert_eq!(client.list_practices().await.unwrap().memberships.len(), 1);
    assert_eq!(
        client
            .get_practice(pid)
            .await
            .unwrap()
            .membership
            .practice
            .id,
        pid
    );
    assert_eq!(
        client
            .create_practice_business(
                pid,
                &CreatePracticeBusinessRequest {
                    name: BusinessName::parse("Client").unwrap()
                }
            )
            .await
            .unwrap()
            .business
            .practice_id,
        pid
    );
    assert_eq!(
        client
            .list_practice_businesses(pid)
            .await
            .unwrap()
            .businesses
            .len(),
        1
    );
    assert_eq!(
        client.list_practice_members(pid).await.unwrap().members[0].role,
        PracticeRole::Member
    );
    client
        .update_practice_member(
            pid,
            uid,
            &UpdatePracticeMemberRequest {
                role: PracticeRole::Admin,
            },
        )
        .await
        .unwrap();
    client.remove_practice_member(pid, uid).await.unwrap();
    assert_eq!(
        client
            .create_practice_invitation(
                pid,
                &CreatePracticeInvitationRequest {
                    email: EmailAddress::parse("person@example.com").unwrap(),
                    role: PracticeRole::Member
                }
            )
            .await
            .unwrap()
            .invitation
            .invitation
            .id,
        iid
    );
    assert_eq!(
        client
            .list_outgoing_practice_invitations(pid)
            .await
            .unwrap()
            .invitations
            .len(),
        1
    );
    client.cancel_practice_invitation(pid, iid).await.unwrap();
    assert_eq!(
        client
            .list_pending_practice_invitations()
            .await
            .unwrap()
            .invitations[0]
            .practice
            .id,
        pid
    );
    client.accept_practice_invitation(iid).await.unwrap();
    client.decline_practice_invitation(iid).await.unwrap();
    let expected = [
        (
            "POST",
            "practices".into(),
            Some(value!({"name":"Accountants"})),
        ),
        ("GET", "practices".into(), None),
        ("GET", format!("practices/{pid}"), None),
        (
            "POST",
            format!("practices/{pid}/businesses"),
            Some(value!({"name":"Client"})),
        ),
        ("GET", format!("practices/{pid}/businesses"), None),
        ("GET", format!("practices/{pid}/members"), None),
        (
            "PATCH",
            format!("practices/{pid}/members/{uid}"),
            Some(value!({"role":"admin"})),
        ),
        ("DELETE", format!("practices/{pid}/members/{uid}"), None),
        (
            "POST",
            format!("practices/{pid}/invitations"),
            Some(value!({"email":"person@example.com","role":"member"})),
        ),
        ("GET", format!("practices/{pid}/invitations"), None),
        ("DELETE", format!("practices/{pid}/invitations/{iid}"), None),
        ("GET", "practice-invitations".into(), None),
        ("POST", format!("practice-invitations/{iid}/accept"), None),
        ("POST", format!("practice-invitations/{iid}/decline"), None),
    ];
    let requests = server.finish();
    assert_eq!(requests.len(), expected.len());
    for (request, (method, path, body)) in requests.iter().zip(expected) {
        assert_request(request, method, &path, body);
    }
}

#[tokio::test]
async fn agreements_keep_business_and_practice_routes_and_states_separate() {
    let subscribed =
        value!({"status":"subscribed","subscribed_at":DATE,"billing_start_month":null});
    let enabled = value!({"status":"enabled","enabled_at":DATE,"billing_start_month":"2026-10","suspension":{"suspended_at":DATE}});
    let server = MockServer::start(vec![
        json(200, &subscribed.to_string()),
        json(200, r#"{"status":"never_subscribed","suspension":null}"#),
        no_content(),
        json(200, &enabled.to_string()),
        json(200, r#"{"status":"never_enabled","suspension":null}"#),
        no_content(),
        json(200, r#"{"payment_method_saved":false}"#),
    ]);
    let client = client(&server);
    let bid = BusinessId::parse(BUSINESS).unwrap();
    let pid = PracticeId::parse(PRACTICE).unwrap();
    assert!(matches!(
        client.subscribe_to_payroll(bid).await.unwrap(),
        SubscribeToPayrollResponse::Subscribed {
            billing_start_month: None,
            ..
        }
    ));
    assert!(matches!(
        client.get_payroll_subscription(bid).await.unwrap(),
        GetPayrollSubscriptionResponse::NeverSubscribed { suspension: None }
    ));
    client.cancel_payroll_subscription(bid).await.unwrap();
    assert!(matches!(
        client.enable_practice_payroll(pid, bid).await.unwrap(),
        PracticePayrollResponse::Enabled {
            suspension: Some(_),
            ..
        }
    ));
    assert!(matches!(
        client.get_practice_payroll(pid, bid).await.unwrap(),
        PracticePayrollResponse::NeverEnabled { suspension: None }
    ));
    client.disable_practice_payroll(pid, bid).await.unwrap();
    assert!(
        !client
            .get_practice_billing_status(pid)
            .await
            .unwrap()
            .payment_method_saved
    );
    let requests = server.finish();
    for (request, method) in requests[..3].iter().zip(["POST", "GET", "DELETE"]) {
        assert_request(
            request,
            method,
            &format!("businesses/{bid}/payroll-subscription"),
            None,
        );
    }
    for (request, method) in requests[3..6].iter().zip(["POST", "GET", "DELETE"]) {
        assert_request(
            request,
            method,
            &format!("practices/{pid}/businesses/{bid}/payroll"),
            None,
        );
    }
    assert_request(
        &requests[6],
        "GET",
        &format!("practices/{pid}/billing/status"),
        None,
    );
}

#[tokio::test]
async fn both_payers_support_payment_selection_invoice_pages_and_pdfs() {
    let methods = value!({"payment_methods":[{"id":"pm_card","type":"card","is_default":true,"card":{"brand":"visa","last4":"4242","expiry_month":12,"expiry_year":2030}},{"id":"pm_bank","type":"bacs_debit","is_default":false,"card":null}],"next_starting_after":"pm_bank"});
    let invoices = value!({"invoices":[{"id":INVOICE,"billing_month":"2026-09","currency":"gbp","subtotal_pence":1234,"created_at":DATE,"status":"paid"}],"next_before":"2026-09"});
    let lines = value!({"lines":[{"business_id":BUSINESS,"business_name":"Client","charge_kind":"payroll_employee","usage_month":"2026-09","quantity":2,"unit_amount_pence":50,"amount_pence":100}],"next_offset":1});
    let mut responses = Vec::new();
    for _ in 0..2 {
        responses.extend([
            json(200, &methods.to_string()),
            no_content(),
            json(200, &invoices.to_string()),
            json(200, &lines.to_string()),
            MockResponse {
                status: 200,
                reason: "OK",
                content_type: Some("application/pdf"),
                body: "%PDF-1.7\nexample\n%%EOF".into(),
            },
        ]);
    }
    let server = MockServer::start(responses);
    let client = client(&server);
    let bid = BusinessId::parse(BUSINESS).unwrap();
    let pid = PracticeId::parse(PRACTICE).unwrap();
    let iid = InvoiceId::parse(INVOICE).unwrap();
    let payment_request = ListPaymentMethodsRequest {
        starting_after: Some(StripePaymentMethodId::parse("pm_previous").unwrap()),
        limit: Some(BillingPageSize::parse(10).unwrap()),
    };
    let selection = SetDefaultPaymentMethodRequest {
        payment_method_id: StripePaymentMethodId::parse("pm_card").unwrap(),
    };
    let invoice_request = ListInvoicesRequest {
        before: Some(BillingMonth::parse("2026-10").unwrap()),
        limit: Some(BillingPageSize::parse(10).unwrap()),
    };
    let lines_request = ListInvoiceLinesRequest {
        offset: Some(0),
        limit: Some(BillingPageSize::parse(10).unwrap()),
    };
    let standalone = client
        .list_payment_methods(bid, &payment_request)
        .await
        .unwrap();
    assert_eq!(
        standalone.payment_methods[1].payment_method_type,
        "bacs_debit"
    );
    client
        .set_default_payment_method(bid, &selection)
        .await
        .unwrap();
    let standalone_invoices = client.list_invoices(bid, &invoice_request).await.unwrap();
    let standalone_lines = client
        .list_invoice_lines(bid, iid, &lines_request)
        .await
        .unwrap();
    let standalone_pdf = client.download_invoice_pdf(bid, iid).await.unwrap();
    assert_eq!(
        client
            .list_practice_payment_methods(pid, &payment_request)
            .await
            .unwrap(),
        standalone
    );
    client
        .set_practice_default_payment_method(pid, &selection)
        .await
        .unwrap();
    assert_eq!(
        client
            .list_practice_invoices(pid, &invoice_request)
            .await
            .unwrap(),
        standalone_invoices
    );
    assert_eq!(
        client
            .list_practice_invoice_lines(pid, iid, &lines_request)
            .await
            .unwrap(),
        standalone_lines
    );
    assert_eq!(
        client
            .download_practice_invoice_pdf(pid, iid)
            .await
            .unwrap(),
        standalone_pdf
    );
    assert_eq!(standalone_pdf, b"%PDF-1.7\nexample\n%%EOF");
    for (requests, base) in server.finish().chunks(5).zip([
        format!("businesses/{bid}/billing"),
        format!("practices/{pid}/billing"),
    ]) {
        assert_request(
            &requests[0],
            "GET",
            &format!("{base}/payment-methods?starting_after=pm_previous&limit=10"),
            None,
        );
        assert_request(
            &requests[1],
            "PUT",
            &format!("{base}/default-payment-method"),
            Some(value!({"payment_method_id":"pm_card"})),
        );
        assert_request(
            &requests[2],
            "GET",
            &format!("{base}/invoices?before=2026-10&limit=10"),
            None,
        );
        assert_request(
            &requests[3],
            "GET",
            &format!("{base}/invoices/{iid}/lines?offset=0&limit=10"),
            None,
        );
        assert_request(
            &requests[4],
            "GET",
            &format!("{base}/invoices/{iid}/pdf"),
            None,
        );
        assert!(requests[4].contains("accept: application/pdf, application/problem+json"));
    }
}

fn round_trip<T: Serialize + DeserializeOwned>(value: Value) {
    let parsed: T = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(parsed).unwrap(), value);
}

#[test]
fn all_agreement_states_round_trip_with_and_without_suspension() {
    for suspension in [Value::Null, value!({"suspended_at":DATE})] {
        round_trip::<GetPayrollSubscriptionResponse>(
            value!({"status":"never_subscribed","suspension":suspension}),
        );
        round_trip::<PracticePayrollResponse>(
            value!({"status":"never_enabled","suspension":suspension}),
        );
        for month in [Value::Null, value!("2026-10")] {
            round_trip::<SubscribeToPayrollResponse>(
                value!({"status":"subscribed","subscribed_at":DATE,"billing_start_month":month}),
            );
            round_trip::<GetPayrollSubscriptionResponse>(
                value!({"status":"subscribed","subscribed_at":DATE,"billing_start_month":month,"suspension":suspension}),
            );
            round_trip::<GetPayrollSubscriptionResponse>(
                value!({"status":"cancelled","subscribed_at":DATE,"billing_start_month":month,"cancelled_at":DATE,"suspension":suspension}),
            );
            round_trip::<PracticePayrollResponse>(
                value!({"status":"enabled","enabled_at":DATE,"billing_start_month":month,"suspension":suspension}),
            );
            round_trip::<PracticePayrollResponse>(
                value!({"status":"disabled","enabled_at":DATE,"billing_start_month":month,"disabled_at":DATE,"suspension":suspension}),
            );
        }
    }
}

#[test]
fn billing_values_validate_and_default_queries_omit_fields() {
    for bad in [
        "2026-00",
        "2026-13",
        "2026-1",
        "2026-01-01",
        "2026-😀",
        " 2026-01",
    ] {
        assert!(BillingMonth::parse(bad).is_err());
    }
    for month in ["2026-01", "2026-12"] {
        round_trip::<BillingMonth>(value!(month));
    }
    for bad in ["pm_", "cus_123", "pm_a/evil", "pm_a_secret_b", "pm_a b"] {
        assert!(StripePaymentMethodId::parse(bad).is_err());
    }
    for bad in [0, 101, u16::MAX] {
        assert!(BillingPageSize::parse(bad).is_err());
    }
    assert_eq!(BillingPageSize::default().get(), 25);
    assert_eq!(
        serde_json::to_value(ListInvoicesRequest::default()).unwrap(),
        value!({})
    );
    assert_eq!(
        serde_json::to_value(ListInvoiceLinesRequest::default()).unwrap(),
        value!({})
    );
    assert_eq!(
        serde_json::to_value(ListPaymentMethodsRequest::default()).unwrap(),
        value!({})
    );
    for status in [
        "pending_delivery",
        "awaiting_status",
        "draft",
        "open",
        "paid",
        "uncollectible",
        "void",
    ] {
        round_trip::<InvoiceStatus>(value!(status));
    }
    assert!(serde_json::from_value::<InvoiceStatus>(value!("unknown")).is_err());
    assert!(
        serde_json::from_value::<PracticePayrollResponse>(value!({"status":"subscribed"})).is_err()
    );
    assert!(
        serde_json::from_value::<GetPayrollSubscriptionResponse>(
            value!({"status":"subscribed","subscribed_at":"bad"})
        )
        .is_err()
    );
    assert!(
        serde_json::from_value::<GetPracticeResponse>(
            value!({"id":PRACTICE,"name":"P","role":"payroll_operator"})
        )
        .is_err()
    );
}

#[tokio::test]
async fn api_errors_survive_json_empty_and_pdf_operations() {
    for status in [400, 401, 403, 404, 409, 502] {
        let problem = value!({"type":"https://api.opsd.sh/problems/test","title":"Test","status":status,"detail":"Denied","category":"request"});
        let server =
            MockServer::start((0..4).map(|_| json(status, &problem.to_string())).collect());
        let client = client(&server);
        let pid = PracticeId::parse(PRACTICE).unwrap();
        let errors = [
            client.get_practice(pid).await.unwrap_err(),
            client
                .disable_practice_payroll(pid, BusinessId::parse(BUSINESS).unwrap())
                .await
                .unwrap_err(),
            client
                .list_practice_invoices(pid, &ListInvoicesRequest::default())
                .await
                .unwrap_err(),
            client
                .download_practice_invoice_pdf(pid, InvoiceId::parse(INVOICE).unwrap())
                .await
                .unwrap_err(),
        ];
        for error in errors {
            match error {
                Error::Api {
                    status: actual,
                    problem,
                } => {
                    assert_eq!(actual.as_u16(), status);
                    assert_eq!(problem.detail, "Denied");
                }
                other => panic!("{other:?}"),
            }
        }
        server.finish();
    }
}

#[tokio::test]
async fn pdf_rejects_successful_non_pdf_responses() {
    for (content_type, body) in [
        ("application/json", "{}"),
        ("application/pdf", "not a PDF"),
        ("text/html", "%PDF-pretend"),
    ] {
        let server = MockServer::start(vec![MockResponse {
            status: 200,
            reason: "OK",
            content_type: Some(content_type),
            body: body.into(),
        }]);
        assert!(matches!(
            client(&server)
                .download_invoice_pdf(
                    BusinessId::parse(BUSINESS).unwrap(),
                    InvoiceId::parse(INVOICE).unwrap()
                )
                .await
                .unwrap_err(),
            Error::UnexpectedResponse { .. }
        ));
        server.finish();
    }
}
