use reqwest::header::ACCEPT;

use super::{ACCEPT_JSON, OpsdClient, decode_response, expect_no_content};
use crate::{
    Error,
    types::{
        AcceptPracticeInvitationResponse, CreatePracticeBusinessRequest,
        CreatePracticeBusinessResponse, CreatePracticeInvitationRequest,
        CreatePracticeInvitationResponse, CreatePracticeRequest, CreatePracticeResponse,
        GetPracticeResponse, ListOutgoingPracticeInvitationsResponse,
        ListPendingPracticeInvitationsResponse, ListPracticeBusinessesResponse,
        ListPracticeMembersResponse, ListPracticesResponse, PracticeId, PracticeInvitationId,
        UpdatePracticeMemberRequest, UserId,
    },
};

impl OpsdClient {
    /// Creates a new memberless client business. Requires practice admin access.
    pub async fn create_practice_business(
        &self,
        practice_id: PracticeId,
        request: &CreatePracticeBusinessRequest,
    ) -> Result<CreatePracticeBusinessResponse, Error> {
        let url = self.endpoint(&format!("practices/{practice_id}/businesses"));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;
        decode_response(response).await
    }

    /// Lists the practice's client businesses for either practice role.
    pub async fn list_practice_businesses(
        &self,
        practice_id: PracticeId,
    ) -> Result<ListPracticeBusinessesResponse, Error> {
        let url = self.endpoint(&format!("practices/{practice_id}/businesses"));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;
        decode_response(response).await
    }

    /// Lists the practices available to the authenticated user.
    pub async fn list_practices(&self) -> Result<ListPracticesResponse, Error> {
        let url = self.endpoint("practices");
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }

    /// Creates a practice and grants the authenticated user admin access.
    pub async fn create_practice(
        &self,
        request: &CreatePracticeRequest,
    ) -> Result<CreatePracticeResponse, Error> {
        let url = self.endpoint("practices");
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;

        decode_response(response).await
    }

    /// Gets a practice available to the authenticated user.
    pub async fn get_practice(
        &self,
        practice_id: PracticeId,
    ) -> Result<GetPracticeResponse, Error> {
        let url = self.endpoint(&format!("practices/{practice_id}"));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }

    /// Lists the users with access to a practice.
    pub async fn list_practice_members(
        &self,
        practice_id: PracticeId,
    ) -> Result<ListPracticeMembersResponse, Error> {
        let url = self.endpoint(&format!("practices/{practice_id}/members"));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }

    /// Changes a practice member's role.
    pub async fn update_practice_member(
        &self,
        practice_id: PracticeId,
        user_id: UserId,
        request: &UpdatePracticeMemberRequest,
    ) -> Result<(), Error> {
        let url = self.endpoint(&format!("practices/{practice_id}/members/{user_id}"));
        let response = self
            .authenticate(self.http_client.patch(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;

        expect_no_content(response).await
    }

    /// Removes a user's access to a practice.
    pub async fn remove_practice_member(
        &self,
        practice_id: PracticeId,
        user_id: UserId,
    ) -> Result<(), Error> {
        let url = self.endpoint(&format!("practices/{practice_id}/members/{user_id}"));
        let response = self
            .authenticate(self.http_client.delete(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        expect_no_content(response).await
    }

    /// Creates a pending invitation to join a practice.
    pub async fn create_practice_invitation(
        &self,
        practice_id: PracticeId,
        request: &CreatePracticeInvitationRequest,
    ) -> Result<CreatePracticeInvitationResponse, Error> {
        let url = self.endpoint(&format!("practices/{practice_id}/invitations"));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .json(request)
            .send()
            .await?;

        decode_response(response).await
    }

    /// Lists pending invitations created for a practice.
    pub async fn list_outgoing_practice_invitations(
        &self,
        practice_id: PracticeId,
    ) -> Result<ListOutgoingPracticeInvitationsResponse, Error> {
        let url = self.endpoint(&format!("practices/{practice_id}/invitations"));
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }

    /// Cancels a pending invitation created for a practice.
    pub async fn cancel_practice_invitation(
        &self,
        practice_id: PracticeId,
        invitation_id: PracticeInvitationId,
    ) -> Result<(), Error> {
        let url = self.endpoint(&format!(
            "practices/{practice_id}/invitations/{invitation_id}"
        ));
        let response = self
            .authenticate(self.http_client.delete(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        expect_no_content(response).await
    }

    /// Lists invitations that the authenticated user can accept or decline.
    pub async fn list_pending_practice_invitations(
        &self,
    ) -> Result<ListPendingPracticeInvitationsResponse, Error> {
        let url = self.endpoint("practice-invitations");
        let response = self
            .authenticate(self.http_client.get(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }

    /// Accepts a pending invitation and returns the joined practice.
    pub async fn accept_practice_invitation(
        &self,
        invitation_id: PracticeInvitationId,
    ) -> Result<AcceptPracticeInvitationResponse, Error> {
        let url = self.endpoint(&format!("practice-invitations/{invitation_id}/accept"));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        decode_response(response).await
    }

    /// Declines a pending invitation.
    pub async fn decline_practice_invitation(
        &self,
        invitation_id: PracticeInvitationId,
    ) -> Result<(), Error> {
        let url = self.endpoint(&format!("practice-invitations/{invitation_id}/decline"));
        let response = self
            .authenticate(self.http_client.post(url))
            .header(ACCEPT, ACCEPT_JSON.clone())
            .send()
            .await?;

        expect_no_content(response).await
    }
}
