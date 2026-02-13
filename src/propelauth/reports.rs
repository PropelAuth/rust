use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::reports::{
    AttritionReportInterval, ChampionReportInterval, ChurnReportInterval, FetchReportQuery,
    GrowthReportInterval, OrgReportType, ReengagementReportInterval, ReportInterval,
    ReportPagination, TopInviterReportInterval, UserReportPage, UserReportType,
};
use crate::propelauth::errors::FetchReportError;

pub struct ReportService<'a> {
    pub(crate) config: &'a Configuration,
}

impl ReportService<'_> {
    pub async fn fetch_user_top_inviter_report(
        &self,
        report_interval: TopInviterReportInterval,
        pagination: ReportPagination,
    ) -> Result<UserReportPage, FetchReportError> {
        let result = crate::apis::report_service_api::fetch_user_report(
            &self.config,
            UserReportType::TopInviter,
            FetchReportQuery {
                report_interval: ReportInterval::TopInviter(report_interval),
                page_size: pagination.page_size,
                page_number: pagination.page_number,
            },
        )
        .await;

        match result {
            Ok(response) => Ok(response),
            Err(Error::ResponseError(response)) => {
                if response.status == 401 {
                    return Err(FetchReportError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(FetchReportError::PropelAuthRateLimit);
                } else {
                    Err(FetchReportError::UnexpectedException)
                }
            }
            Err(_) => Err(FetchReportError::UnexpectedException),
        }
    }

    pub async fn fetch_user_champion_report(
        &self,
        report_interval: ChampionReportInterval,
        pagination: ReportPagination,
    ) -> Result<UserReportPage, FetchReportError> {
        let result = crate::apis::report_service_api::fetch_user_report(
            &self.config,
            UserReportType::Champion,
            FetchReportQuery {
                report_interval: ReportInterval::Champion(report_interval),
                page_size: pagination.page_size,
                page_number: pagination.page_number,
            },
        )
        .await;

        match result {
            Ok(response) => Ok(response),
            Err(Error::ResponseError(response)) => {
                if response.status == 401 {
                    return Err(FetchReportError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(FetchReportError::PropelAuthRateLimit);
                } else {
                    Err(FetchReportError::UnexpectedException)
                }
            }
            Err(_) => Err(FetchReportError::UnexpectedException),
        }
    }

    pub async fn fetch_user_reengagement_report(
        &self,
        report_interval: ReengagementReportInterval,
        pagination: ReportPagination,
    ) -> Result<UserReportPage, FetchReportError> {
        let result = crate::apis::report_service_api::fetch_user_report(
            &self.config,
            UserReportType::Reengagement,
            FetchReportQuery {
                report_interval: ReportInterval::Reengagement(report_interval),
                page_size: pagination.page_size,
                page_number: pagination.page_number,
            },
        )
        .await;

        match result {
            Ok(response) => Ok(response),
            Err(Error::ResponseError(response)) => {
                if response.status == 401 {
                    return Err(FetchReportError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(FetchReportError::PropelAuthRateLimit);
                } else {
                    Err(FetchReportError::UnexpectedException)
                }
            }
            Err(_) => Err(FetchReportError::UnexpectedException),
        }
    }

    pub async fn fetch_user_churn_report(
        &self,
        report_interval: ChurnReportInterval,
        pagination: ReportPagination,
    ) -> Result<UserReportPage, FetchReportError> {
        let result = crate::apis::report_service_api::fetch_user_report(
            &self.config,
            UserReportType::Churn,
            FetchReportQuery {
                report_interval: ReportInterval::Churn(report_interval),
                page_size: pagination.page_size,
                page_number: pagination.page_number,
            },
        )
        .await;

        match result {
            Ok(response) => Ok(response),
            Err(Error::ResponseError(response)) => {
                if response.status == 401 {
                    return Err(FetchReportError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(FetchReportError::PropelAuthRateLimit);
                } else {
                    Err(FetchReportError::UnexpectedException)
                }
            }
            Err(_) => Err(FetchReportError::UnexpectedException),
        }
    }

    pub async fn fetch_org_attrition_report(
        &self,
        report_interval: AttritionReportInterval,
        pagination: ReportPagination,
    ) -> Result<crate::models::reports::OrgReport, FetchReportError> {
        let result = crate::apis::report_service_api::fetch_org_report(
            &self.config,
            OrgReportType::Attrition,
            FetchReportQuery {
                report_interval: ReportInterval::Attrition(report_interval),
                page_size: pagination.page_size,
                page_number: pagination.page_number,
            },
        )
        .await;

        match result {
            Ok(response) => Ok(response),
            Err(Error::ResponseError(response)) => {
                if response.status == 401 {
                    return Err(FetchReportError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(FetchReportError::PropelAuthRateLimit);
                } else {
                    Err(FetchReportError::UnexpectedException)
                }
            }
            Err(_) => Err(FetchReportError::UnexpectedException),
        }
    }

    pub async fn fetch_org_growth_report(
        &self,
        report_interval: GrowthReportInterval,
        pagination: ReportPagination,
    ) -> Result<crate::models::reports::OrgReport, FetchReportError> {
        let result = crate::apis::report_service_api::fetch_org_report(
            &self.config,
            OrgReportType::Growth,
            FetchReportQuery {
                report_interval: ReportInterval::Growth(report_interval),
                page_size: pagination.page_size,
                page_number: pagination.page_number,
            },
        )
        .await;

        match result {
            Ok(response) => Ok(response),
            Err(Error::ResponseError(response)) => {
                if response.status == 401 {
                    return Err(FetchReportError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(FetchReportError::PropelAuthRateLimit);
                } else {
                    Err(FetchReportError::UnexpectedException)
                }
            }
            Err(_) => Err(FetchReportError::UnexpectedException),
        }
    }

    pub async fn fetch_org_reengagement_report(
        &self,
        report_interval: ReengagementReportInterval,
        pagination: ReportPagination,
    ) -> Result<crate::models::reports::OrgReport, FetchReportError> {
        let result = crate::apis::report_service_api::fetch_org_report(
            &self.config,
            OrgReportType::Reengagement,
            FetchReportQuery {
                report_interval: ReportInterval::Reengagement(report_interval),
                page_size: pagination.page_size,
                page_number: pagination.page_number,
            },
        )
        .await;

        match result {
            Ok(response) => Ok(response),
            Err(Error::ResponseError(response)) => {
                if response.status == 401 {
                    return Err(FetchReportError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(FetchReportError::PropelAuthRateLimit);
                } else {
                    Err(FetchReportError::UnexpectedException)
                }
            }
            Err(_) => Err(FetchReportError::UnexpectedException),
        }
    }

    pub async fn fetch_org_churn_report(
        &self,
        report_interval: ChurnReportInterval,
        pagination: ReportPagination,
    ) -> Result<crate::models::reports::OrgReport, FetchReportError> {
        let result = crate::apis::report_service_api::fetch_org_report(
            &self.config,
            OrgReportType::Churn,
            FetchReportQuery {
                report_interval: ReportInterval::Churn(report_interval),
                page_size: pagination.page_size,
                page_number: pagination.page_number,
            },
        )
        .await;

        match result {
            Ok(response) => Ok(response),
            Err(Error::ResponseError(response)) => {
                if response.status == 401 {
                    return Err(FetchReportError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(FetchReportError::PropelAuthRateLimit);
                } else {
                    Err(FetchReportError::UnexpectedException)
                }
            }
            Err(_) => Err(FetchReportError::UnexpectedException),
        }
    }
}
