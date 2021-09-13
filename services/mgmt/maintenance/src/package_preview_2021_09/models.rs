#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplyUpdateProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<apply_update_properties::Status>,
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "lastUpdateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
}
pub mod apply_update_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Pending,
        InProgress,
        Completed,
        RetryNow,
        RetryLater,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplyUpdate {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplyUpdateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListApplyUpdate {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApplyUpdate>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationAssignmentProperties {
    #[serde(rename = "maintenanceConfigurationId", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_configuration_id: Option<String>,
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationAssignment {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationAssignmentProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListConfigurationAssignmentsResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConfigurationAssignment>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    #[serde(rename = "startDateTime", default, skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<String>,
    #[serde(rename = "expirationDateTime", default, skip_serializing_if = "Option::is_none")]
    pub expiration_date_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(rename = "recurEvery", default, skip_serializing_if = "Option::is_none")]
    pub recur_every: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceConfigurationProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "extensionProperties", default, skip_serializing_if = "Option::is_none")]
    pub extension_properties: Option<serde_json::Value>,
    #[serde(rename = "maintenanceScope", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_scope: Option<maintenance_configuration_properties::MaintenanceScope>,
    #[serde(rename = "maintenanceWindow", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<MaintenanceWindow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<maintenance_configuration_properties::Visibility>,
    #[serde(rename = "installPatches", default, skip_serializing_if = "Option::is_none")]
    pub install_patches: Option<InputPatchConfiguration>,
}
pub mod maintenance_configuration_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MaintenanceScope {
        Host,
        #[serde(rename = "OSImage")]
        OsImage,
        Extension,
        InGuestPatch,
        #[serde(rename = "SQLDB")]
        Sqldb,
        #[serde(rename = "SQLManagedInstance")]
        SqlManagedInstance,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Visibility {
        Custom,
        Public,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputPatchConfiguration {
    #[serde(rename = "rebootSetting", default, skip_serializing_if = "Option::is_none")]
    pub reboot_setting: Option<input_patch_configuration::RebootSetting>,
    #[serde(rename = "windowsParameters", default, skip_serializing_if = "Option::is_none")]
    pub windows_parameters: Option<InputWindowsParameters>,
    #[serde(rename = "linuxParameters", default, skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<InputLinuxParameters>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasks: Option<SoftwareUpdateConfigurationTasks>,
}
pub mod input_patch_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RebootSetting {
        NeverReboot,
        RebootIfRequired,
        AlwaysReboot,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputWindowsParameters {
    #[serde(rename = "kbNumbersToExclude", default, skip_serializing_if = "Vec::is_empty")]
    pub kb_numbers_to_exclude: Vec<String>,
    #[serde(rename = "kbNumbersToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub kb_numbers_to_include: Vec<String>,
    #[serde(rename = "classificationsToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub classifications_to_include: Vec<String>,
    #[serde(rename = "excludeKbsRequiringReboot", default, skip_serializing_if = "Option::is_none")]
    pub exclude_kbs_requiring_reboot: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputLinuxParameters {
    #[serde(rename = "packageNameMasksToExclude", default, skip_serializing_if = "Vec::is_empty")]
    pub package_name_masks_to_exclude: Vec<String>,
    #[serde(rename = "packageNameMasksToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub package_name_masks_to_include: Vec<String>,
    #[serde(rename = "classificationsToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub classifications_to_include: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoftwareUpdateConfigurationTasks {
    #[serde(rename = "preTasks", default, skip_serializing_if = "Vec::is_empty")]
    pub pre_tasks: Vec<TaskProperties>,
    #[serde(rename = "postTasks", default, skip_serializing_if = "Vec::is_empty")]
    pub post_tasks: Vec<TaskProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "taskScope", default, skip_serializing_if = "Option::is_none")]
    pub task_scope: Option<task_properties::TaskScope>,
}
pub mod task_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TaskScope {
        Global,
        Resource,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceConfiguration {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MaintenanceConfigurationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListMaintenanceConfigurationsResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MaintenanceConfiguration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListUpdatesResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Update>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Update {
    #[serde(rename = "maintenanceScope", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_scope: Option<update::MaintenanceScope>,
    #[serde(rename = "impactType", default, skip_serializing_if = "Option::is_none")]
    pub impact_type: Option<update::ImpactType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<update::Status>,
    #[serde(rename = "impactDurationInSec", default, skip_serializing_if = "Option::is_none")]
    pub impact_duration_in_sec: Option<i32>,
    #[serde(rename = "notBefore", default, skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateProperties>,
}
pub mod update {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MaintenanceScope {
        Host,
        #[serde(rename = "OSImage")]
        OsImage,
        Extension,
        InGuestPatch,
        #[serde(rename = "SQLDB")]
        Sqldb,
        #[serde(rename = "SQLManagedInstance")]
        SqlManagedInstance,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ImpactType {
        None,
        Freeze,
        Restart,
        Redeploy,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Pending,
        InProgress,
        Completed,
        RetryNow,
        RetryLater,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateProperties {
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemData {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
pub mod system_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}