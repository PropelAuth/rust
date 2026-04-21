use crate::apis::configuration::Configuration;
use crate::apis::mfa_service_api::{SendSmsMfaCodeParams, VerifyTotpChallengeParams};
use crate::apis::mfa_service_api::{VerifySmsChallengeParams, VerifyStepUpGrantParams};
use crate::apis::Error;
use crate::models::VerifyTotpChallengeResponse;
use crate::models::{SendSmsCodeResponse, VerifySmsChallengeResponse, VerifyStepUpGrantResponse};
use crate::propelauth::errors::{
    SendSmsCodeError, VerifySmsChallengeError, VerifyStepUpGrantError,
    VerifyStepUpTotpChallengeError,
};

pub struct MfaService<'a> {
    pub(crate) config: &'a Configuration,
}

impl MfaService<'_> {
    pub async fn verify_step_up_totp_challenge(
        &self,
        params: VerifyTotpChallengeParams,
    ) -> Result<VerifyTotpChallengeResponse, VerifyStepUpTotpChallengeError> {
        let result =
            crate::apis::mfa_service_api::verify_step_up_totp_challenge(&self.config, params).await;

        match result {
            Ok(response) => Ok(response),
            Err(Error::ResponseError(response)) => {
                if response.status == 401 {
                    return Err(VerifyStepUpTotpChallengeError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(VerifyStepUpTotpChallengeError::PropelAuthRateLimit);
                }

                let error_response: Result<serde_json::Value, _> =
                    serde_json::from_str(&response.content);
                if let Ok(error_json) = error_response {
                    if let Some(error_code) = error_json.get("error_code").and_then(|v| v.as_str())
                    {
                        match error_code {
                            "user_not_found" => {
                                return Err(VerifyStepUpTotpChallengeError::UserNotFound)
                            }
                            "mfa_not_enabled" => {
                                return Err(VerifyStepUpTotpChallengeError::MfaNotEnabled)
                            }
                            "incorrect_mfa_code" => {
                                return Err(VerifyStepUpTotpChallengeError::IncorrectMfaCode)
                            }
                            "invalid_request_fields" => {
                                return Err(VerifyStepUpTotpChallengeError::BadRequest(
                                    response.content,
                                ))
                            }
                            "feature_gated" => {
                                return Err(VerifyStepUpTotpChallengeError::FeatureGated)
                            }
                            _ => {}
                        }
                    }
                }

                Err(VerifyStepUpTotpChallengeError::UnexpectedException)
            }
            Err(_) => Err(VerifyStepUpTotpChallengeError::UnexpectedException),
        }
    }

    pub async fn verify_step_up_grant(
        &self,
        params: VerifyStepUpGrantParams,
    ) -> Result<VerifyStepUpGrantResponse, VerifyStepUpGrantError> {
        let result = crate::apis::mfa_service_api::verify_step_up_grant(&self.config, params).await;

        match result {
            Ok(_) => Ok(VerifyStepUpGrantResponse { success: true }),
            Err(Error::ResponseError(response)) => {
                if response.status == 401 {
                    return Err(VerifyStepUpGrantError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(VerifyStepUpGrantError::PropelAuthRateLimit);
                }

                let error_response: Result<serde_json::Value, _> =
                    serde_json::from_str(&response.content);
                if let Ok(error_json) = error_response {
                    if let Some(error_code) = error_json.get("error_code").and_then(|v| v.as_str())
                    {
                        match error_code {
                            "invalid_request_fields" => {
                                if let Some(field_to_errors) = error_json
                                    .get("field_to_errors")
                                    .and_then(|v| v.as_object())
                                {
                                    if let Some(grant_error) =
                                        field_to_errors.get("grant").and_then(|v| v.as_str())
                                    {
                                        if grant_error == "grant_not_found" {
                                            return Ok(VerifyStepUpGrantResponse {
                                                success: false,
                                            });
                                        }
                                    }
                                }

                                return Err(VerifyStepUpGrantError::BadRequest(response.content));
                            }
                            "feature_gated" => return Err(VerifyStepUpGrantError::FeatureGated),
                            _ => {}
                        }
                    }
                }

                Err(VerifyStepUpGrantError::UnexpectedException)
            }
            Err(_) => Err(VerifyStepUpGrantError::UnexpectedException),
        }
    }

    pub async fn send_sms_mfa_code(
        &self,
        params: SendSmsMfaCodeParams,
    ) -> Result<SendSmsCodeResponse, SendSmsCodeError> {
        let result = crate::apis::mfa_service_api::send_sms_mfa_code(&self.config, params).await;

        match result {
            Ok(response) => Ok(response),
            Err(Error::ResponseError(response)) => {
                if response.status == 401 {
                    return Err(SendSmsCodeError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(SendSmsCodeError::PropelAuthRateLimit);
                }

                let error_response: Result<serde_json::Value, _> =
                    serde_json::from_str(&response.content);
                if let Ok(error_json) = error_response {
                    if let Some(error_code) = error_json.get("error_code").and_then(|v| v.as_str())
                    {
                        match error_code {
                            "user_not_found" => return Err(SendSmsCodeError::UserNotFound),
                            "mfa_not_enabled" => return Err(SendSmsCodeError::MfaNotEnabled),
                            "invalid_request_fields" => {
                                return Err(SendSmsCodeError::BadRequest(response.content))
                            }
                            "feature_gated" => return Err(SendSmsCodeError::FeatureGated),
                            _ => {}
                        }
                    }
                }
                Err(SendSmsCodeError::UnexpectedException)
            }
            Err(_) => Err(SendSmsCodeError::UnexpectedException),
        }
    }

    pub async fn verify_sms_challenge(
        &self,
        params: VerifySmsChallengeParams,
    ) -> Result<VerifySmsChallengeResponse, VerifySmsChallengeError> {
        let result = crate::apis::mfa_service_api::verify_sms_challenge(&self.config, params).await;

        match result {
            Ok(response) => Ok(response),
            Err(Error::ResponseError(response)) => {
                if response.status == 401 {
                    return Err(VerifySmsChallengeError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(VerifySmsChallengeError::PropelAuthRateLimit);
                }

                let error_response: Result<serde_json::Value, _> =
                    serde_json::from_str(&response.content);
                if let Ok(error_json) = error_response {
                    if let Some(error_code) = error_json.get("error_code").and_then(|v| v.as_str())
                    {
                        match error_code {
                            "user_not_found" => return Err(VerifySmsChallengeError::UserNotFound),
                            "mfa_not_enabled" => {
                                return Err(VerifySmsChallengeError::MfaNotEnabled)
                            }
                            "invalid_request_fields" => {
                                return Err(VerifySmsChallengeError::BadRequest(response.content))
                            }
                            "feature_gated" => return Err(VerifySmsChallengeError::FeatureGated),
                            _ => {}
                        }
                    }
                }
                Err(VerifySmsChallengeError::UnexpectedException)
            }
            Err(_) => Err(VerifySmsChallengeError::UnexpectedException),
        }
    }
}
