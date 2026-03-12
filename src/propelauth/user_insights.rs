use crate::apis::configuration::Configuration;
use crate::apis::{user_insights_service_api, Error};
use crate::models::user_insights::{
    AttritionReportInterval, ChampionReportInterval, ChartMetric, ChurnReportInterval,
    FetchChartDataQuery, FetchReportQuery, GrowthReportInterval, OrgReportType,
    ReengagementReportInterval, ReportInterval, ReportPagination, TopInviterReportInterval,
    UserReportPage, UserReportType,
};
use crate::propelauth::errors::FetchUserInsightsError;

pub struct UserInsightsService<'a> {
    pub(crate) config: &'a Configuration,
}

impl UserInsightsService<'_> {
    pub async fn fetch_user_top_inviter_report(
        &self,
        report_interval: TopInviterReportInterval,
        pagination: ReportPagination,
    ) -> Result<UserReportPage, FetchUserInsightsError> {
        let result = user_insights_service_api::fetch_user_report(
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
                    return Err(FetchUserInsightsError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(FetchUserInsightsError::PropelAuthRateLimit);
                } else {
                    Err(FetchUserInsightsError::UnexpectedException)
                }
            }
            Err(_) => Err(FetchUserInsightsError::UnexpectedException),
        }
    }

    pub async fn fetch_user_champion_report(
        &self,
        report_interval: ChampionReportInterval,
        pagination: ReportPagination,
    ) -> Result<UserReportPage, FetchUserInsightsError> {
        let result = user_insights_service_api::fetch_user_report(
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
                    return Err(FetchUserInsightsError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(FetchUserInsightsError::PropelAuthRateLimit);
                } else {
                    Err(FetchUserInsightsError::UnexpectedException)
                }
            }
            Err(_) => Err(FetchUserInsightsError::UnexpectedException),
        }
    }

    pub async fn fetch_user_reengagement_report(
        &self,
        report_interval: ReengagementReportInterval,
        pagination: ReportPagination,
    ) -> Result<UserReportPage, FetchUserInsightsError> {
        let result = user_insights_service_api::fetch_user_report(
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
                    return Err(FetchUserInsightsError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(FetchUserInsightsError::PropelAuthRateLimit);
                } else {
                    Err(FetchUserInsightsError::UnexpectedException)
                }
            }
            Err(_) => Err(FetchUserInsightsError::UnexpectedException),
        }
    }

    pub async fn fetch_user_churn_report(
        &self,
        report_interval: ChurnReportInterval,
        pagination: ReportPagination,
    ) -> Result<UserReportPage, FetchUserInsightsError> {
        let result = user_insights_service_api::fetch_user_report(
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
                    return Err(FetchUserInsightsError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(FetchUserInsightsError::PropelAuthRateLimit);
                } else {
                    Err(FetchUserInsightsError::UnexpectedException)
                }
            }
            Err(_) => Err(FetchUserInsightsError::UnexpectedException),
        }
    }

    pub async fn fetch_org_attrition_report(
        &self,
        report_interval: AttritionReportInterval,
        pagination: ReportPagination,
    ) -> Result<crate::models::user_insights::OrgReport, FetchUserInsightsError> {
        let result = user_insights_service_api::fetch_org_report(
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
                    return Err(FetchUserInsightsError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(FetchUserInsightsError::PropelAuthRateLimit);
                } else {
                    Err(FetchUserInsightsError::UnexpectedException)
                }
            }
            Err(_) => Err(FetchUserInsightsError::UnexpectedException),
        }
    }

    pub async fn fetch_org_growth_report(
        &self,
        report_interval: GrowthReportInterval,
        pagination: ReportPagination,
    ) -> Result<crate::models::user_insights::OrgReport, FetchUserInsightsError> {
        let result = user_insights_service_api::fetch_org_report(
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
                    return Err(FetchUserInsightsError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(FetchUserInsightsError::PropelAuthRateLimit);
                } else {
                    Err(FetchUserInsightsError::UnexpectedException)
                }
            }
            Err(_) => Err(FetchUserInsightsError::UnexpectedException),
        }
    }

    pub async fn fetch_org_reengagement_report(
        &self,
        report_interval: ReengagementReportInterval,
        pagination: ReportPagination,
    ) -> Result<crate::models::user_insights::OrgReport, FetchUserInsightsError> {
        let result = user_insights_service_api::fetch_org_report(
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
                    return Err(FetchUserInsightsError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(FetchUserInsightsError::PropelAuthRateLimit);
                } else {
                    Err(FetchUserInsightsError::UnexpectedException)
                }
            }
            Err(_) => Err(FetchUserInsightsError::UnexpectedException),
        }
    }

    pub async fn fetch_org_churn_report(
        &self,
        report_interval: ChurnReportInterval,
        pagination: ReportPagination,
    ) -> Result<crate::models::user_insights::OrgReport, FetchUserInsightsError> {
        let result = user_insights_service_api::fetch_org_report(
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
                    return Err(FetchUserInsightsError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(FetchUserInsightsError::PropelAuthRateLimit);
                } else {
                    Err(FetchUserInsightsError::UnexpectedException)
                }
            }
            Err(_) => Err(FetchUserInsightsError::UnexpectedException),
        }
    }

    pub async fn fetch_chart_metric_data(
        &self,
        chart_metric: ChartMetric,
        params: FetchChartDataQuery,
    ) -> Result<crate::models::user_insights::ChartData, FetchUserInsightsError> {
        if let Some(start_date_raw) = &params.start_date {
            if let Ok(start_date) = chrono::NaiveDate::parse_from_str(start_date_raw, "%Y-%m-%d") {
                if start_date > chrono::Utc::now().naive_utc().date() {
                    return Err(FetchUserInsightsError::InvalidParams(
                        "start_date cannot be in the future",
                    ));
                }
            } else {
                return Err(FetchUserInsightsError::InvalidParams(
                    "start_date must be in YYYY-MM-DD format",
                ));
            }
        }

        if let Some(end_date_raw) = &params.end_date {
            if chrono::NaiveDate::parse_from_str(end_date_raw, "%Y-%m-%d").is_err() {
                return Err(FetchUserInsightsError::InvalidParams(
                    "end_date must be in YYYY-MM-DD format",
                ));
            }
        }

        let result =
            user_insights_service_api::fetch_chart_metric_data(&self.config, chart_metric, params)
                .await;

        match result {
            Ok(response) => Ok(response),
            Err(Error::ResponseError(response)) => {
                if response.status == 401 {
                    return Err(FetchUserInsightsError::InvalidApiKey);
                } else if response.status == 429 {
                    return Err(FetchUserInsightsError::PropelAuthRateLimit);
                } else {
                    Err(FetchUserInsightsError::UnexpectedException)
                }
            }
            Err(_) => Err(FetchUserInsightsError::UnexpectedException),
        }
    }
}
