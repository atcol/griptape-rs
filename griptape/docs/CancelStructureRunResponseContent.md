# CancelStructureRunResponseContent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**structure_run_id** | **String** |  | 
**structure_id** | **String** |  | 
**created_by** | **String** |  | 
**created_at** | **String** |  | 
**started_at** | **String** |  | 
**updated_at** | **String** |  | 
**completed_at** | **String** |  | 
**status** | [**models::StructureRunStatus**](StructureRunStatus.md) |  | 
**status_detail** | Option<[**serde_json::Value**](.md)> |  | [optional]
**args** | **Vec<String>** |  | 
**env** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**env_vars** | Option<[**Vec<models::EnvVar>**](EnvVar.md)> |  | [optional]
**output** | Option<[**serde_json::Value**](.md)> |  | [optional]
**output_timestamp** | Option<**f64**> |  | [optional]
**deployment_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


