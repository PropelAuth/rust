use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ReportPagination {
    pub page_size: Option<i32>,
    pub page_number: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct FetchReportQuery {
    pub(crate) report_interval: ReportInterval,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page_number: Option<i32>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OrgReportType {
    Attrition,
    Growth,
    Reengagement,
    Churn,
}

impl OrgReportType {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrgReportType::Attrition => "attrition",
            OrgReportType::Growth => "growth",
            OrgReportType::Reengagement => "reengagement",
            OrgReportType::Churn => "churn",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UserReportType {
    TopInviter,
    Champion,
    Reengagement,
    Churn,
}

impl UserReportType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserReportType::TopInviter => "top_inviter",
            UserReportType::Champion => "champion",
            UserReportType::Reengagement => "reengagement",
            UserReportType::Churn => "churn",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub(crate) enum ReportInterval {
    TopInviter(TopInviterReportInterval),
    Champion(ChampionReportInterval),
    Reengagement(ReengagementReportInterval),
    Churn(ChurnReportInterval),
    Attrition(AttritionReportInterval),
    Growth(GrowthReportInterval),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum TopInviterReportInterval {
    #[serde(rename = "30")]
    ThirtyDays,
    #[serde(rename = "60")]
    SixtyDays,
    #[serde(rename = "90")]
    NinetyDays,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ChampionReportInterval {
    #[serde(rename = "30")]
    ThirtyDays,
    #[serde(rename = "60")]
    SixtyDays,
    #[serde(rename = "90")]
    NinetyDays,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ReengagementReportInterval {
    #[serde(rename = "Weekly")]
    Weekly,
    #[serde(rename = "Monthly")]
    Monthly,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ChurnReportInterval {
    #[serde(rename = "7")]
    SevenDays,
    #[serde(rename = "14")]
    FourteenDays,
    #[serde(rename = "30")]
    ThirtyDays,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum AttritionReportInterval {
    #[serde(rename = "30")]
    ThirtyDays,
    #[serde(rename = "60")]
    SixtyDays,
    #[serde(rename = "90")]
    NinetyDays,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum GrowthReportInterval {
    #[serde(rename = "30")]
    ThirtyDays,
    #[serde(rename = "60")]
    SixtyDays,
    #[serde(rename = "90")]
    NinetyDays,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrgReportRecord {
    pub id: String,
    pub report_id: String,
    pub org_id: String,
    pub name: String,
    pub num_users: i32,
    pub org_created_at: i64,
    pub extra_properties: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct OrgReport {
    pub org_reports: Vec<OrgReportRecord>,
    pub current_page: i64,
    pub total_count: i64,
    pub page_size: i64,
    pub has_more_results: bool,
    pub report_time: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserReportRecord {
    pub id: String,
    pub report_id: String,
    pub user_id: String,
    pub user_created_at: i64,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub last_active_at: i64,
    pub org_data: serde_json::Value,
    pub extra_properties: Option<serde_json::Value>,
}
#[derive(Deserialize)]
pub struct UserReportPage {
    pub user_reports: Vec<UserReportRecord>,
    pub current_page: i64,
    pub total_count: i64,
    pub page_size: i64,
    pub has_more_results: bool,
    pub report_time: i64,
}
