use reqwest::header::ACCEPT;

use super::{ACCEPT_JSON, OpsdClient, decode_response};
use crate::{
    Error,
    types::{
        BusinessId, CreateEmployeeRequest, CreateEmployeeResponse, EmployeeId, GetEmployeeResponse,
        ListEmployeesResponse, UpdateEmployeeRequest, UpdateEmployeeResponse,
    },
};

impl OpsdClient {
    /// Creates an employee belonging to a business.
    pub async fn create_employee(
        &self,
        business_id: BusinessId,
        request: &CreateEmployeeRequest,
    ) -> Result<CreateEmployeeResponse, Error> {
        let url = self.endpoint(&format!("businesses/{business_id}/employees"));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;

        decode_response(response).await
    }

    /// Lists employees belonging to a business.
    pub async fn list_employees(
        &self,
        business_id: BusinessId,
    ) -> Result<ListEmployeesResponse, Error> {
        let url = self.endpoint(&format!("businesses/{business_id}/employees"));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }

    /// Gets an employee belonging to a business.
    pub async fn get_employee(
        &self,
        business_id: BusinessId,
        employee_id: EmployeeId,
    ) -> Result<GetEmployeeResponse, Error> {
        let url = self.endpoint(&format!("businesses/{business_id}/employees/{employee_id}"));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }

    /// Changes an employee's name.
    pub async fn update_employee(
        &self,
        business_id: BusinessId,
        employee_id: EmployeeId,
        request: &UpdateEmployeeRequest,
    ) -> Result<UpdateEmployeeResponse, Error> {
        let url = self.endpoint(&format!("businesses/{business_id}/employees/{employee_id}"));
        let response = self
            .authenticate(self.http_client.patch(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;

        decode_response(response).await
    }
}
