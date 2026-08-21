use tonic::transport::Channel;

use crate::{
    accounts::handlers::{LoginBody, RegisterBody, ValidateTokenBody},
    app_error::AppError,
    pb::auth::{
        AuthResponse, LoginRequest, RegisterRequest, ValidateTokenRequest, ValidateTokenResponse,
        auth_service_client::AuthServiceClient,
    },
};

#[derive(Clone)]
pub struct AuthService {
    pub client: AuthServiceClient<Channel>,
}

impl AuthService {
    pub fn new(client: AuthServiceClient<Channel>) -> Self {
        Self { client }
    }

    pub async fn login(self, payload: LoginBody) -> Result<AuthResponse, AppError> {
        let mut client = self.client.clone();
        let login_response = client
            .login(LoginRequest {
                email: payload.email,
                password: payload.password,
            })
            .await?;
        Ok(login_response.into_inner())
    }
    pub async fn register(self, payload: RegisterBody) -> Result<AuthResponse, AppError> {
        let mut client = self.client.clone();
        let login_response = client
            .register(RegisterRequest {
                email: payload.email,
                password: payload.password,
                username: payload.username,
            })
            .await?;
        Ok(login_response.into_inner())
    }
    pub async fn validate_token(
        self,
        payload: ValidateTokenBody,
    ) -> Result<ValidateTokenResponse, AppError> {
        let mut client = self.client.clone();
        let login_response = client
            .validate_token(ValidateTokenRequest {
                token: payload.token,
            })
            .await?;
        Ok(login_response.into_inner())
    }
}
