# AssistantRunDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assistant_run_id** | **String** |  | 
**assistant_id** | **String** |  | 
**created_by** | **String** |  | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**completed_at** | **String** |  | 
**status** | [**models::AssistantRunStatus**](AssistantRunStatus.md) |  | 
**status_detail** | Option<[**serde_json::Value**](.md)> |  | [optional]
**input** | Option<**String**> |  | [optional]
**args** | **Vec<String>** |  | 
**output** | Option<[**serde_json::Value**](.md)> |  | [optional]
**thread_id** | Option<**String**> |  | [optional]
**knowledge_base_ids** | **Vec<String>** |  | 
**ruleset_ids** | **Vec<String>** |  | 
**stream** | **bool** |  | 
**structure_ids** | **Vec<String>** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


