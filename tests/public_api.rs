#![cfg(feature = "client")]

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str::FromStr,
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
};

use opsd::{
    ApiCredential, OpsdClient,
    types::{
        AccountsOfficeReference, BusinessId, BusinessInvitationId, BusinessName, BusinessRole,
        CreateBusinessInvitationRequest, CreateBusinessRequest, CreateEmployeeRequest,
        CreateEmploymentRequest, CreatePayeSchemeRequest, CreatePayrollRunRequest, EmailAddress,
        EmployeeForenames, EmployeeId, EmployeeSurname, EmployerReference, EmploymentId,
        PayeSchemeId, PayeSchemeName, PayrollPaymentDate, PayrollRunId,
        UpdateBusinessMemberRequest, UpdateEmployeeRequest, UserId,
    },
};
use url::Url;

const BUSINESS: &str =
    r#"{"id":"22222222-2222-4222-8222-222222222222","name":"Example Ltd","role":"admin"}"#;
const MEMBER: &str = r#"{"id":"33333333-3333-4333-8333-333333333333","email":"member@example.com","email_verified":true,"role":"payroll_operator"}"#;
const OUTGOING_INVITATION: &str = r#"{"id":"44444444-4444-4444-8444-444444444444","email":"invitee@example.com","role":"payroll_operator","status":"pending","expires_at":"2026-09-11T12:00:00Z"}"#;
const PENDING_INVITATION: &str = r#"{"id":"44444444-4444-4444-8444-444444444444","business_id":"22222222-2222-4222-8222-222222222222","business_name":"Example Ltd","invited_by_email":"admin@example.com","role":"payroll_operator","expires_at":"2026-09-11T12:00:00Z"}"#;
const EMPLOYEE: &str = r#"{"id":"55555555-5555-4555-8555-555555555555","forenames":["Jamie","Lee"],"surname":"Morgan","created_at":"2026-09-04T12:00:00Z"}"#;
const PAYE_SCHEME: &str = r#"{"id":"66666666-6666-4666-8666-666666666666","name":"Monthly payroll","employer_reference":"123/AB456","accounts_office_reference":"123PA00045678","created_at":"2026-09-04T12:00:00Z"}"#;
const EMPLOYMENT: &str = r#"{"id":"77777777-7777-4777-8777-777777777777","employee_id":"55555555-5555-4555-8555-555555555555","paye_scheme_id":"66666666-6666-4666-8666-666666666666","created_at":"2026-09-04T12:00:00Z"}"#;
const PAYROLL_RUN: &str = r#"{"id":"88888888-8888-4888-8888-888888888888","paye_scheme_id":"66666666-6666-4666-8666-666666666666","payment_date":"2026-09-30","status":"draft","employment_count":1,"finalized_at":null,"finalized_by_user_id":null,"created_at":"2026-09-04T12:00:00Z"}"#;
const PAYROLL_RUN_DETAILS: &str = r#"{"id":"88888888-8888-4888-8888-888888888888","paye_scheme_id":"66666666-6666-4666-8666-666666666666","payment_date":"2026-09-30","status":"finalized","employment_count":1,"finalized_at":"2026-09-04T13:00:00Z","finalized_by_user_id":"33333333-3333-4333-8333-333333333333","created_at":"2026-09-04T12:00:00Z","employment_ids":["77777777-7777-4777-8777-777777777777"]}"#;

#[tokio::test]
async fn client_covers_the_public_business_and_payroll_api() {
    let responses = vec![
        json(201, BUSINESS),
        json(200, &format!("[{BUSINESS}]")),
        json(200, BUSINESS),
        json(200, &format!("[{MEMBER}]")),
        no_content(),
        no_content(),
        json(201, OUTGOING_INVITATION),
        json(200, &format!("[{OUTGOING_INVITATION}]")),
        no_content(),
        json(200, &format!("[{PENDING_INVITATION}]")),
        json(200, BUSINESS),
        no_content(),
        json(201, EMPLOYEE),
        json(200, &format!("[{EMPLOYEE}]")),
        json(200, EMPLOYEE),
        json(200, EMPLOYEE),
        json(201, PAYE_SCHEME),
        json(200, &format!("[{PAYE_SCHEME}]")),
        json(201, EMPLOYMENT),
        json(200, &format!("[{EMPLOYMENT}]")),
        json(201, PAYROLL_RUN),
        json(200, &format!("[{PAYROLL_RUN}]")),
        json(200, PAYROLL_RUN_DETAILS),
        json(200, PAYROLL_RUN_DETAILS),
        no_content(),
        no_content(),
    ];
    let server = MockServer::start(responses);
    let client = OpsdClient::new_base(
        server.base_url.clone(),
        ApiCredential::new("test-token").unwrap(),
    )
    .unwrap();

    let business_id = BusinessId::from_str("22222222-2222-4222-8222-222222222222").unwrap();
    let user_id = UserId::from_str("33333333-3333-4333-8333-333333333333").unwrap();
    let invitation_id =
        BusinessInvitationId::from_str("44444444-4444-4444-8444-444444444444").unwrap();
    let employee_id = EmployeeId::from_str("55555555-5555-4555-8555-555555555555").unwrap();
    let paye_scheme_id = PayeSchemeId::from_str("66666666-6666-4666-8666-666666666666").unwrap();
    let employment_id = EmploymentId::from_str("77777777-7777-4777-8777-777777777777").unwrap();
    let payroll_run_id = PayrollRunId::from_str("88888888-8888-4888-8888-888888888888").unwrap();

    client
        .create_business(&CreateBusinessRequest {
            name: BusinessName::parse("Example Ltd").unwrap(),
        })
        .await
        .unwrap();
    client.list_businesses().await.unwrap();
    client.get_business(business_id).await.unwrap();
    client.list_business_members(business_id).await.unwrap();
    client
        .update_business_member(
            business_id,
            user_id,
            &UpdateBusinessMemberRequest {
                role: BusinessRole::PayrollOperator,
            },
        )
        .await
        .unwrap();
    client
        .remove_business_member(business_id, user_id)
        .await
        .unwrap();
    client
        .create_business_invitation(
            business_id,
            &CreateBusinessInvitationRequest {
                email: EmailAddress::parse("invitee@example.com").unwrap(),
                role: BusinessRole::PayrollOperator,
            },
        )
        .await
        .unwrap();
    client
        .list_outgoing_business_invitations(business_id)
        .await
        .unwrap();
    client
        .cancel_business_invitation(business_id, invitation_id)
        .await
        .unwrap();
    client.list_pending_business_invitations().await.unwrap();
    client
        .accept_business_invitation(invitation_id)
        .await
        .unwrap();
    client
        .decline_business_invitation(invitation_id)
        .await
        .unwrap();
    client
        .create_employee(
            business_id,
            &CreateEmployeeRequest {
                forenames: EmployeeForenames::parse(vec!["Jamie".to_string(), "Lee".to_string()])
                    .unwrap(),
                surname: EmployeeSurname::parse("Morgan").unwrap(),
            },
        )
        .await
        .unwrap();
    client.list_employees(business_id).await.unwrap();
    client.get_employee(business_id, employee_id).await.unwrap();
    client
        .update_employee(
            business_id,
            employee_id,
            &UpdateEmployeeRequest {
                forenames: None,
                surname: Some(EmployeeSurname::parse("Taylor").unwrap()),
            },
        )
        .await
        .unwrap();
    client
        .create_paye_scheme(
            business_id,
            &CreatePayeSchemeRequest {
                name: PayeSchemeName::parse("Monthly payroll").unwrap(),
                employer_reference: EmployerReference::parse("123/AB456").unwrap(),
                accounts_office_reference: AccountsOfficeReference::parse("123PA00045678").unwrap(),
            },
        )
        .await
        .unwrap();
    client.list_paye_schemes(business_id).await.unwrap();
    client
        .create_employment(
            business_id,
            &CreateEmploymentRequest {
                employee_id,
                paye_scheme_id,
            },
        )
        .await
        .unwrap();
    client.list_employments(business_id).await.unwrap();
    client
        .create_payroll_run(
            business_id,
            &CreatePayrollRunRequest {
                paye_scheme_id,
                payment_date: PayrollPaymentDate::parse("2026-09-30").unwrap(),
            },
        )
        .await
        .unwrap();
    client.list_payroll_runs(business_id).await.unwrap();
    client
        .get_payroll_run(business_id, payroll_run_id)
        .await
        .unwrap();
    client
        .finalize_payroll_run(business_id, payroll_run_id)
        .await
        .unwrap();
    client
        .include_payroll_run_employment(business_id, payroll_run_id, employment_id)
        .await
        .unwrap();
    client
        .exclude_payroll_run_employment(business_id, payroll_run_id, employment_id)
        .await
        .unwrap();

    let requests = server.finish();
    assert_request_lines(&requests);
    assert_json_body(
        &requests[0],
        serde_json::json!({
            "name": "Example Ltd",
        }),
    );
    assert_json_body(
        &requests[4],
        serde_json::json!({
            "role": "payroll_operator",
        }),
    );
    assert_json_body(
        &requests[6],
        serde_json::json!({
            "email": "invitee@example.com",
            "role": "payroll_operator",
        }),
    );
    assert_json_body(
        &requests[12],
        serde_json::json!({
            "forenames": ["Jamie", "Lee"],
            "surname": "Morgan",
        }),
    );
    assert_json_body(
        &requests[15],
        serde_json::json!({
            "surname": "Taylor",
        }),
    );
    assert_json_body(
        &requests[16],
        serde_json::json!({
            "name": "Monthly payroll",
            "employer_reference": "123/AB456",
            "accounts_office_reference": "123PA00045678",
        }),
    );
    assert_json_body(
        &requests[18],
        serde_json::json!({
            "employee_id": "55555555-5555-4555-8555-555555555555",
            "paye_scheme_id": "66666666-6666-4666-8666-666666666666",
        }),
    );
    assert_json_body(
        &requests[20],
        serde_json::json!({
            "paye_scheme_id": "66666666-6666-4666-8666-666666666666",
            "payment_date": "2026-09-30",
        }),
    );
}

fn assert_json_body(request: &str, expected: serde_json::Value) {
    let (_, body) = request
        .split_once("\r\n\r\n")
        .expect("request must contain an HTTP header terminator");
    let actual: serde_json::Value =
        serde_json::from_str(body).expect("request body must contain valid JSON");

    assert_eq!(actual, expected);
}

fn assert_request_lines(requests: &[String]) {
    let expected = [
        "POST /v1/businesses HTTP/1.1",
        "GET /v1/businesses HTTP/1.1",
        "GET /v1/businesses/22222222-2222-4222-8222-222222222222 HTTP/1.1",
        "GET /v1/businesses/22222222-2222-4222-8222-222222222222/members HTTP/1.1",
        "PATCH /v1/businesses/22222222-2222-4222-8222-222222222222/members/33333333-3333-4333-8333-333333333333 HTTP/1.1",
        "DELETE /v1/businesses/22222222-2222-4222-8222-222222222222/members/33333333-3333-4333-8333-333333333333 HTTP/1.1",
        "POST /v1/businesses/22222222-2222-4222-8222-222222222222/invitations HTTP/1.1",
        "GET /v1/businesses/22222222-2222-4222-8222-222222222222/invitations HTTP/1.1",
        "DELETE /v1/businesses/22222222-2222-4222-8222-222222222222/invitations/44444444-4444-4444-8444-444444444444 HTTP/1.1",
        "GET /v1/business-invitations HTTP/1.1",
        "POST /v1/business-invitations/44444444-4444-4444-8444-444444444444/accept HTTP/1.1",
        "POST /v1/business-invitations/44444444-4444-4444-8444-444444444444/decline HTTP/1.1",
        "POST /v1/businesses/22222222-2222-4222-8222-222222222222/employees HTTP/1.1",
        "GET /v1/businesses/22222222-2222-4222-8222-222222222222/employees HTTP/1.1",
        "GET /v1/businesses/22222222-2222-4222-8222-222222222222/employees/55555555-5555-4555-8555-555555555555 HTTP/1.1",
        "PATCH /v1/businesses/22222222-2222-4222-8222-222222222222/employees/55555555-5555-4555-8555-555555555555 HTTP/1.1",
        "POST /v1/businesses/22222222-2222-4222-8222-222222222222/paye-schemes HTTP/1.1",
        "GET /v1/businesses/22222222-2222-4222-8222-222222222222/paye-schemes HTTP/1.1",
        "POST /v1/businesses/22222222-2222-4222-8222-222222222222/employments HTTP/1.1",
        "GET /v1/businesses/22222222-2222-4222-8222-222222222222/employments HTTP/1.1",
        "POST /v1/businesses/22222222-2222-4222-8222-222222222222/payroll-runs HTTP/1.1",
        "GET /v1/businesses/22222222-2222-4222-8222-222222222222/payroll-runs HTTP/1.1",
        "GET /v1/businesses/22222222-2222-4222-8222-222222222222/payroll-runs/88888888-8888-4888-8888-888888888888 HTTP/1.1",
        "POST /v1/businesses/22222222-2222-4222-8222-222222222222/payroll-runs/88888888-8888-4888-8888-888888888888/finalize HTTP/1.1",
        "PUT /v1/businesses/22222222-2222-4222-8222-222222222222/payroll-runs/88888888-8888-4888-8888-888888888888/employments/77777777-7777-4777-8777-777777777777 HTTP/1.1",
        "DELETE /v1/businesses/22222222-2222-4222-8222-222222222222/payroll-runs/88888888-8888-4888-8888-888888888888/employments/77777777-7777-4777-8777-777777777777 HTTP/1.1",
    ];

    assert_eq!(requests.len(), expected.len());
    for (request, expected_line) in requests.iter().zip(expected) {
        assert_eq!(request.lines().next(), Some(expected_line));
        assert!(request.contains("authorization: Bearer test-token\r\n"));
    }
}

struct MockServer {
    base_url: Url,
    requests: Receiver<String>,
    thread: JoinHandle<()>,
}

impl MockServer {
    fn start(responses: Vec<MockResponse>) -> Self {
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let address = listener.local_addr().unwrap();
        let (sender, requests) = mpsc::channel();
        let thread = thread::spawn(move || {
            for response in responses {
                let (mut stream, _) = listener.accept().unwrap();
                sender.send(read_request(&mut stream)).unwrap();
                stream.write_all(&response.as_bytes()).unwrap();
            }
        });

        Self {
            base_url: Url::parse(&format!("http://{address}/v1/")).unwrap(),
            requests,
            thread,
        }
    }

    fn finish(self) -> Vec<String> {
        self.thread.join().unwrap();
        self.requests.into_iter().collect()
    }
}

struct MockResponse {
    status: u16,
    reason: &'static str,
    content_type: Option<&'static str>,
    body: String,
}

impl MockResponse {
    fn as_bytes(&self) -> Vec<u8> {
        let content_type = self
            .content_type
            .map(|value| format!("Content-Type: {value}\r\n"))
            .unwrap_or_default();

        format!(
            "HTTP/1.1 {} {}\r\n{}Content-Length: {}\r\nConnection: close\r\n\r\n{}",
            self.status,
            self.reason,
            content_type,
            self.body.len(),
            self.body
        )
        .into_bytes()
    }
}

fn json(status: u16, body: &str) -> MockResponse {
    MockResponse {
        status,
        reason: if status == 201 { "Created" } else { "OK" },
        content_type: Some("application/json"),
        body: body.to_string(),
    }
}

fn no_content() -> MockResponse {
    MockResponse {
        status: 204,
        reason: "No Content",
        content_type: None,
        body: String::new(),
    }
}

fn read_request(stream: &mut TcpStream) -> String {
    let mut request = Vec::new();
    let mut buffer = [0_u8; 4096];

    loop {
        let read = stream.read(&mut buffer).unwrap();
        assert_ne!(read, 0, "client closed request before sending its body");
        request.extend_from_slice(&buffer[..read]);

        if let Some(header_end) = find_bytes(&request, b"\r\n\r\n") {
            let body_start = header_end + 4;
            let headers = String::from_utf8_lossy(&request[..header_end]);
            let content_length = headers
                .lines()
                .find_map(|line| {
                    line.to_ascii_lowercase()
                        .strip_prefix("content-length: ")
                        .and_then(|value| value.parse::<usize>().ok())
                })
                .unwrap_or(0);

            if request.len() >= body_start + content_length {
                return String::from_utf8(request).unwrap();
            }
        }
    }
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}
