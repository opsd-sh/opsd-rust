//! Billing bearer APIs. All operations require the payer's administrator role.
//! Business routes are for standalone businesses; use practice routes for
//! practice-owned clients. Website cookie-based portal setup is not exposed.

use reqwest::{
    Method,
    header::{ACCEPT, CONTENT_TYPE},
};
use serde::{Serialize, de::DeserializeOwned};

use super::{ACCEPT_JSON, OpsdClient, decode_error, decode_response, expect_no_content};
use crate::{Error, types::*};

impl OpsdClient {
    /// Checks the standalone business's default payment method live in Stripe.
    /// This does not guarantee that a future charge succeeds.
    pub async fn get_billing_status(
        &self,
        business_id: BusinessId,
    ) -> Result<GetBillingStatusResponse, Error> {
        self.billing_get(&format!("businesses/{business_id}/billing/status"), &())
            .await
    }

    /// Checks the practice's default payment method live in Stripe.
    pub async fn get_practice_billing_status(
        &self,
        practice_id: PracticeId,
    ) -> Result<GetBillingStatusResponse, Error> {
        self.billing_get(&format!("practices/{practice_id}/billing/status"), &())
            .await
    }

    /// Starts or resumes a standalone payroll agreement. This alone does not
    /// start billing or clear an overdue-invoice suspension.
    pub async fn subscribe_to_payroll(
        &self,
        business_id: BusinessId,
    ) -> Result<SubscribeToPayrollResponse, Error> {
        self.billing_post(&format!("businesses/{business_id}/payroll-subscription"))
            .await
    }

    /// Reads agreement status and independent suspension for a standalone payer.
    pub async fn get_payroll_subscription(
        &self,
        business_id: BusinessId,
    ) -> Result<GetPayrollSubscriptionResponse, Error> {
        self.billing_get(
            &format!("businesses/{business_id}/payroll-subscription"),
            &(),
        )
        .await
    }

    /// Cancels a standalone agreement without clearing outstanding charges.
    pub async fn cancel_payroll_subscription(&self, business_id: BusinessId) -> Result<(), Error> {
        self.billing_delete(&format!("businesses/{business_id}/payroll-subscription"))
            .await
    }

    /// Enables payroll for a practice-owned client. Does not lift suspension.
    pub async fn enable_practice_payroll(
        &self,
        practice_id: PracticeId,
        business_id: BusinessId,
    ) -> Result<PracticePayrollResponse, Error> {
        self.billing_post(&format!(
            "practices/{practice_id}/businesses/{business_id}/payroll"
        ))
        .await
    }

    /// Reads client payroll status, including any practice-wide suspension.
    pub async fn get_practice_payroll(
        &self,
        practice_id: PracticeId,
        business_id: BusinessId,
    ) -> Result<PracticePayrollResponse, Error> {
        self.billing_get(
            &format!("practices/{practice_id}/businesses/{business_id}/payroll"),
            &(),
        )
        .await
    }

    /// Disables client payroll without clearing the practice's outstanding charges.
    pub async fn disable_practice_payroll(
        &self,
        practice_id: PracticeId,
        business_id: BusinessId,
    ) -> Result<(), Error> {
        self.billing_delete(&format!(
            "practices/{practice_id}/businesses/{business_id}/payroll"
        ))
        .await
    }

    /// Lists the standalone payer's live saved payment methods.
    pub async fn list_payment_methods(
        &self,
        business_id: BusinessId,
        request: &ListPaymentMethodsRequest,
    ) -> Result<ListPaymentMethodsResponse, Error> {
        self.billing_get(
            &format!("businesses/{business_id}/billing/payment-methods"),
            request,
        )
        .await
    }

    /// Lists the practice payer's live saved payment methods.
    pub async fn list_practice_payment_methods(
        &self,
        practice_id: PracticeId,
        request: &ListPaymentMethodsRequest,
    ) -> Result<ListPaymentMethodsResponse, Error> {
        self.billing_get(
            &format!("practices/{practice_id}/billing/payment-methods"),
            request,
        )
        .await
    }

    /// Selects an existing payment method without charging or retrying invoices.
    pub async fn set_default_payment_method(
        &self,
        business_id: BusinessId,
        request: &SetDefaultPaymentMethodRequest,
    ) -> Result<(), Error> {
        self.billing_put(
            &format!("businesses/{business_id}/billing/default-payment-method"),
            request,
        )
        .await
    }

    /// Selects an existing practice payment method. Does not clear suspension.
    pub async fn set_practice_default_payment_method(
        &self,
        practice_id: PracticeId,
        request: &SetDefaultPaymentMethodRequest,
    ) -> Result<(), Error> {
        self.billing_put(
            &format!("practices/{practice_id}/billing/default-payment-method"),
            request,
        )
        .await
    }

    /// Lists saved standalone invoices, newest first (not live Stripe balances).
    pub async fn list_invoices(
        &self,
        business_id: BusinessId,
        request: &ListInvoicesRequest,
    ) -> Result<ListInvoicesResponse, Error> {
        self.billing_get(
            &format!("businesses/{business_id}/billing/invoices"),
            request,
        )
        .await
    }

    /// Lists consolidated saved practice invoices, newest first.
    pub async fn list_practice_invoices(
        &self,
        practice_id: PracticeId,
        request: &ListInvoicesRequest,
    ) -> Result<ListInvoicesResponse, Error> {
        self.billing_get(
            &format!("practices/{practice_id}/billing/invoices"),
            request,
        )
        .await
    }

    /// Reads saved detailed charges for a standalone invoice.
    pub async fn list_invoice_lines(
        &self,
        business_id: BusinessId,
        invoice_id: InvoiceId,
        request: &ListInvoiceLinesRequest,
    ) -> Result<ListInvoiceLinesResponse, Error> {
        self.billing_get(
            &format!("businesses/{business_id}/billing/invoices/{invoice_id}/lines"),
            request,
        )
        .await
    }

    /// Reads detailed charges with the originating business for each line.
    pub async fn list_practice_invoice_lines(
        &self,
        practice_id: PracticeId,
        invoice_id: InvoiceId,
        request: &ListInvoiceLinesRequest,
    ) -> Result<ListInvoiceLinesResponse, Error> {
        self.billing_get(
            &format!("practices/{practice_id}/billing/invoices/{invoice_id}/lines"),
            request,
        )
        .await
    }

    /// Downloads invoice PDF bytes through Opsd. A not-yet-available PDF returns
    /// the API's conflict error; this method never opens a browser or writes a file.
    pub async fn download_invoice_pdf(
        &self,
        business_id: BusinessId,
        invoice_id: InvoiceId,
    ) -> Result<Vec<u8>, Error> {
        self.billing_pdf(&format!(
            "businesses/{business_id}/billing/invoices/{invoice_id}/pdf"
        ))
        .await
    }

    /// Downloads the consolidated PDF. Use list_practice_invoice_lines for the
    /// full per-client breakdown, rather than the PDF's summary lines.
    pub async fn download_practice_invoice_pdf(
        &self,
        practice_id: PracticeId,
        invoice_id: InvoiceId,
    ) -> Result<Vec<u8>, Error> {
        self.billing_pdf(&format!(
            "practices/{practice_id}/billing/invoices/{invoice_id}/pdf"
        ))
        .await
    }

    async fn billing_get<T: DeserializeOwned, Q: Serialize + ?Sized>(
        &self,
        path: &str,
        query: &Q,
    ) -> Result<T, Error> {
        let response = self
            .authenticate(self.http_client.get(self.endpoint(path)))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .query(query)
            .send()
            .await?;
        decode_response(response).await
    }

    async fn billing_post<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let response = self
            .authenticate(self.http_client.post(self.endpoint(path)))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;
        decode_response(response).await
    }

    async fn billing_delete(&self, path: &str) -> Result<(), Error> {
        let response = self
            .authenticate(self.http_client.delete(self.endpoint(path)))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;
        expect_no_content(response).await
    }

    async fn billing_put<T: Serialize + ?Sized>(&self, path: &str, body: &T) -> Result<(), Error> {
        let response = self
            .authenticate(self.http_client.request(Method::PUT, self.endpoint(path)))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(body)
            .send()
            .await?;
        expect_no_content(response).await
    }

    async fn billing_pdf(&self, path: &str) -> Result<Vec<u8>, Error> {
        let response = self
            .authenticate(self.http_client.get(self.endpoint(path)))
            .header(ACCEPT, "application/pdf, application/problem+json")
            .send()
            .await?;
        let status = response.status();
        let is_pdf = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| {
                value
                    .split(';')
                    .next()
                    .unwrap_or("")
                    .trim()
                    .eq_ignore_ascii_case("application/pdf")
            });
        let body = response.bytes().await?;
        if !status.is_success() {
            return Err(decode_error(status, &body));
        }
        if status != reqwest::StatusCode::OK || !is_pdf || !body.starts_with(b"%PDF-") {
            return Err(Error::UnexpectedResponse {
                status,
                body: String::from_utf8_lossy(&body).into_owned(),
            });
        }
        Ok(body.to_vec())
    }
}
