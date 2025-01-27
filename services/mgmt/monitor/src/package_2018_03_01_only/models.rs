#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroupResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionGroup>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ActionGroupResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroup {
    #[serde(rename = "groupShortName")]
    pub group_short_name: String,
    pub enabled: bool,
    #[serde(rename = "emailReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub email_receivers: Vec<EmailReceiver>,
    #[serde(rename = "smsReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub sms_receivers: Vec<SmsReceiver>,
    #[serde(rename = "webhookReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub webhook_receivers: Vec<WebhookReceiver>,
    #[serde(rename = "itsmReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub itsm_receivers: Vec<ItsmReceiver>,
    #[serde(rename = "azureAppPushReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_app_push_receivers: Vec<AzureAppPushReceiver>,
    #[serde(rename = "automationRunbookReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub automation_runbook_receivers: Vec<AutomationRunbookReceiver>,
    #[serde(rename = "voiceReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub voice_receivers: Vec<VoiceReceiver>,
    #[serde(rename = "logicAppReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub logic_app_receivers: Vec<LogicAppReceiver>,
    #[serde(rename = "azureFunctionReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_function_receivers: Vec<AzureFunctionReceiver>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailReceiver {
    pub name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceiverStatus>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsReceiver {
    pub name: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceiverStatus>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookReceiver {
    pub name: String,
    #[serde(rename = "serviceUri")]
    pub service_uri: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItsmReceiver {
    pub name: String,
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    #[serde(rename = "ticketConfiguration")]
    pub ticket_configuration: String,
    pub region: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureAppPushReceiver {
    pub name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationRunbookReceiver {
    #[serde(rename = "automationAccountId")]
    pub automation_account_id: String,
    #[serde(rename = "runbookName")]
    pub runbook_name: String,
    #[serde(rename = "webhookResourceId")]
    pub webhook_resource_id: String,
    #[serde(rename = "isGlobalRunbook")]
    pub is_global_runbook: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "serviceUri", default, skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoiceReceiver {
    pub name: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogicAppReceiver {
    pub name: String,
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFunctionReceiver {
    pub name: String,
    #[serde(rename = "functionAppResourceId")]
    pub function_app_resource_id: String,
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[serde(rename = "httpTriggerUrl")]
    pub http_trigger_url: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReceiverStatus {
    NotSpecified,
    Enabled,
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnableRequest {
    #[serde(rename = "receiverName")]
    pub receiver_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroupPatchBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionGroupPatch>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroupPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertAction {
    #[serde(rename = "actionGroupId", default, skip_serializing_if = "Option::is_none")]
    pub action_group_id: Option<String>,
    #[serde(rename = "webHookProperties", default, skip_serializing_if = "Option::is_none")]
    pub web_hook_properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub severity: i32,
    pub enabled: bool,
    pub scopes: Vec<String>,
    #[serde(rename = "evaluationFrequency")]
    pub evaluation_frequency: String,
    #[serde(rename = "windowSize")]
    pub window_size: String,
    #[serde(rename = "targetResourceType", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_type: Option<String>,
    #[serde(rename = "targetResourceRegion", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_region: Option<String>,
    pub criteria: MetricAlertCriteria,
    #[serde(rename = "autoMitigate", default, skip_serializing_if = "Option::is_none")]
    pub auto_mitigate: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<MetricAlertAction>,
    #[serde(rename = "lastUpdatedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    #[serde(rename = "isMigrated", default, skip_serializing_if = "Option::is_none")]
    pub is_migrated: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertPropertiesPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    #[serde(rename = "evaluationFrequency", default, skip_serializing_if = "Option::is_none")]
    pub evaluation_frequency: Option<String>,
    #[serde(rename = "windowSize", default, skip_serializing_if = "Option::is_none")]
    pub window_size: Option<String>,
    #[serde(rename = "targetResourceType", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_type: Option<String>,
    #[serde(rename = "targetResourceRegion", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criteria: Option<MetricAlertCriteria>,
    #[serde(rename = "autoMitigate", default, skip_serializing_if = "Option::is_none")]
    pub auto_mitigate: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<MetricAlertAction>,
    #[serde(rename = "lastUpdatedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    #[serde(rename = "isMigrated", default, skip_serializing_if = "Option::is_none")]
    pub is_migrated: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: MetricAlertProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertResourcePatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetricAlertPropertiesPatch>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertResourceCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MetricAlertResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertStatusCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MetricAlertStatus>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertStatusProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetricAlertStatusProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertCriteria {
    #[serde(rename = "odata.type")]
    pub odata_type: metric_alert_criteria::Odata_type,
}
pub mod metric_alert_criteria {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Odata_type {
        #[serde(rename = "Microsoft.Azure.Monitor.SingleResourceMultipleMetricCriteria")]
        MicrosoftAzureMonitorSingleResourceMultipleMetricCriteria,
        #[serde(rename = "Microsoft.Azure.Monitor.MultipleResourceMultipleMetricCriteria")]
        MicrosoftAzureMonitorMultipleResourceMultipleMetricCriteria,
        #[serde(rename = "Microsoft.Azure.Monitor.WebtestLocationAvailabilityCriteria")]
        MicrosoftAzureMonitorWebtestLocationAvailabilityCriteria,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertSingleResourceMultipleMetricCriteria {
    #[serde(flatten)]
    pub metric_alert_criteria: MetricAlertCriteria,
    #[serde(rename = "allOf", default, skip_serializing_if = "Vec::is_empty")]
    pub all_of: Vec<MetricCriteria>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebtestLocationAvailabilityCriteria {
    #[serde(flatten)]
    pub metric_alert_criteria: MetricAlertCriteria,
    #[serde(rename = "webTestId")]
    pub web_test_id: String,
    #[serde(rename = "componentId")]
    pub component_id: String,
    #[serde(rename = "failedLocationCount")]
    pub failed_location_count: f64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricCriteria {
    #[serde(flatten)]
    pub multi_metric_criteria: MultiMetricCriteria,
    pub operator: metric_criteria::Operator,
    pub threshold: f64,
}
pub mod metric_criteria {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        Equals,
        GreaterThan,
        GreaterThanOrEqual,
        LessThan,
        LessThanOrEqual,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDimension {
    pub name: String,
    pub operator: String,
    pub values: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAlertMultipleResourceMultipleMetricCriteria {
    #[serde(flatten)]
    pub metric_alert_criteria: MetricAlertCriteria,
    #[serde(rename = "allOf", default, skip_serializing_if = "Vec::is_empty")]
    pub all_of: Vec<MultiMetricCriteria>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiMetricCriteria {
    #[serde(rename = "criterionType")]
    pub criterion_type: multi_metric_criteria::CriterionType,
    pub name: String,
    #[serde(rename = "metricName")]
    pub metric_name: String,
    #[serde(rename = "metricNamespace", default, skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
    #[serde(rename = "timeAggregation")]
    pub time_aggregation: multi_metric_criteria::TimeAggregation,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<MetricDimension>,
    #[serde(rename = "skipMetricValidation", default, skip_serializing_if = "Option::is_none")]
    pub skip_metric_validation: Option<bool>,
}
pub mod multi_metric_criteria {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CriterionType {
        StaticThresholdCriterion,
        DynamicThresholdCriterion,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TimeAggregation {
        Average,
        Count,
        Minimum,
        Maximum,
        Total,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamicMetricCriteria {
    #[serde(flatten)]
    pub multi_metric_criteria: MultiMetricCriteria,
    pub operator: dynamic_metric_criteria::Operator,
    #[serde(rename = "alertSensitivity")]
    pub alert_sensitivity: dynamic_metric_criteria::AlertSensitivity,
    #[serde(rename = "failingPeriods")]
    pub failing_periods: DynamicThresholdFailingPeriods,
    #[serde(rename = "ignoreDataBefore", default, skip_serializing_if = "Option::is_none")]
    pub ignore_data_before: Option<String>,
}
pub mod dynamic_metric_criteria {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        GreaterThan,
        LessThan,
        GreaterOrLessThan,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AlertSensitivity {
        Low,
        Medium,
        High,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamicThresholdFailingPeriods {
    #[serde(rename = "numberOfEvaluationPeriods")]
    pub number_of_evaluation_periods: f64,
    #[serde(rename = "minFailingPeriodsToAlert")]
    pub min_failing_periods_to_alert: f64,
}
