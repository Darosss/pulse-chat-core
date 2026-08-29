use tonic::transport::Channel;

use crate::{
    accounts::handlers::{LoginBody, LogoutBody, RegisterBody},
    app_error::AppError,
    pb::auth::{
        AuthResponse, LoginRequest, LogoutRequest, LogoutResponse, RegisterRequest,
        auth::{GetPublicJwtKeyRequest, GetPublicJwtKeyResponse},
        auth_service_client::AuthServiceClient,
    },
};

#[derive(Clone)]
pub struct AuthService {
    pub client: AuthServiceClient<Channel>,
}

impl AuthService {
    pub fn new(channel: Channel) -> Self {
        Self {
            client: AuthServiceClient::new(channel),
        }
    }

    pub async fn login(self, payload: LoginBody) -> Result<AuthResponse, AppError> {
        let mut client = self.client.clone();
        let response = client
            .login(LoginRequest {
                email: payload.email,
                password: payload.password,
            })
            .await?;
        Ok(response.into_inner())
    }
    pub async fn logout(self, payload: LogoutBody) -> Result<LogoutResponse, AppError> {
        let mut client = self.client.clone();
        let response = client
            .logout(LogoutRequest {
                refresh_token: payload.refresh_token,
            })
            .await?;
        Ok(response.into_inner())
    }
    pub async fn register(self, payload: RegisterBody) -> Result<AuthResponse, AppError> {
        let mut client = self.client.clone();
        let response = client
            .register(RegisterRequest {
                email: payload.email,
                password: payload.password,
                username: payload.username,
            })
            .await?;
        Ok(response.into_inner())
    }

    pub async fn get_public_jwt_key(self) -> Result<GetPublicJwtKeyResponse, AppError> {
        let mut client = self.client.clone();
        let response = client.get_public_jwt_key(GetPublicJwtKeyRequest {}).await?;
        Ok(response.into_inner())
    }
}
