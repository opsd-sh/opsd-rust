use reqwest::header::ACCEPT;

use super::{ACCEPT_JSON, OpsdClient, decode_response, expect_no_content};
use crate::{
    Error,
    types::{
        AcceptBusinessInvitationResponse, BusinessId, BusinessInvitationId,
        CreateBusinessInvitationRequest, CreateBusinessInvitationResponse, CreateBusinessRequest,
        CreateBusinessResponse, GetBusinessResponse, ListBusinessMembersResponse,
        ListBusinessesResponse, ListOutgoingBusinessInvitationsResponse,
        ListPendingBusinessInvitationsResponse, UpdateBusinessMemberRequest, UserId,
    },
};

impl OpsdClient {
    /// Lists the businesses available to the authenticated user.
    pub async fn list_businesses(&self) -> Result<ListBusinessesResponse, Error> {
        let url = self.endpoint("businesses");
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }

    /// Creates a business and grants the authenticated user admin access.
    pub async fn create_business(
        &self,
        request: &CreateBusinessRequest,
    ) -> Result<CreateBusinessResponse, Error> {
        let url = self.endpoint("businesses");
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;

        decode_response(response).await
    }

    /// Gets a business available to the authenticated user.
    pub async fn get_business(
        &self,
        business_id: BusinessId,
    ) -> Result<GetBusinessResponse, Error> {
        let url = self.endpoint(&format!("businesses/{business_id}"));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }

    /// Lists the users with access to a business.
    pub async fn list_business_members(
        &self,
        business_id: BusinessId,
    ) -> Result<ListBusinessMembersResponse, Error> {
        let url = self.endpoint(&format!("businesses/{business_id}/members"));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }

    /// Changes a business member's role.
    pub async fn update_business_member(
        &self,
        business_id: BusinessId,
        user_id: UserId,
        request: &UpdateBusinessMemberRequest,
    ) -> Result<(), Error> {
        let url = self.endpoint(&format!("businesses/{business_id}/members/{user_id}"));
        let response = self
            .authenticate(self.http_client.patch(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;

        expect_no_content(response).await
    }

    /// Removes a user's access to a business.
    pub async fn remove_business_member(
        &self,
        business_id: BusinessId,
        user_id: UserId,
    ) -> Result<(), Error> {
        let url = self.endpoint(&format!("businesses/{business_id}/members/{user_id}"));
        let response = self
            .authenticate(self.http_client.delete(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        expect_no_content(response).await
    }

    /// Creates a pending invitation to join a business.
    pub async fn create_business_invitation(
        &self,
        business_id: BusinessId,
        request: &CreateBusinessInvitationRequest,
    ) -> Result<CreateBusinessInvitationResponse, Error> {
        let url = self.endpoint(&format!("businesses/{business_id}/invitations"));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;

        decode_response(response).await
    }

    /// Lists pending invitations created for a business.
    pub async fn list_outgoing_business_invitations(
        &self,
        business_id: BusinessId,
    ) -> Result<ListOutgoingBusinessInvitationsResponse, Error> {
        let url = self.endpoint(&format!("businesses/{business_id}/invitations"));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }

    /// Cancels a pending invitation created for a business.
    pub async fn cancel_business_invitation(
        &self,
        business_id: BusinessId,
        invitation_id: BusinessInvitationId,
    ) -> Result<(), Error> {
        let url = self.endpoint(&format!(
            "businesses/{business_id}/invitations/{invitation_id}"
        ));
        let response = self
            .authenticate(self.http_client.delete(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        expect_no_content(response).await
    }

    /// Lists invitations that the authenticated user can accept or decline.
    pub async fn list_pending_business_invitations(
        &self,
    ) -> Result<ListPendingBusinessInvitationsResponse, Error> {
        let url = self.endpoint("business-invitations");
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }

    /// Accepts a pending invitation and returns the joined business.
    pub async fn accept_business_invitation(
        &self,
        invitation_id: BusinessInvitationId,
    ) -> Result<AcceptBusinessInvitationResponse, Error> {
        let url = self.endpoint(&format!("business-invitations/{invitation_id}/accept"));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }

    /// Declines a pending invitation.
    pub async fn decline_business_invitation(
        &self,
        invitation_id: BusinessInvitationId,
    ) -> Result<(), Error> {
        let url = self.endpoint(&format!("business-invitations/{invitation_id}/decline"));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        expect_no_content(response).await
    }
}
