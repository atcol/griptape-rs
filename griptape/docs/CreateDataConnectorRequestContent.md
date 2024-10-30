# CreateDataConnectorRequestContent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**config** | [**models::DataConnectorConfigInputUnion**](DataConnectorConfigInputUnion.md) |  | 
**description** | Option<**String**> |  | [optional]
**schedule_expression** | Option<**String**> |  | [optional]
**r#type** | **String** |  | 
**embedding_model** | Option<[**models::EmbeddingModel**](EmbeddingModel.md)> |  | [optional]
**use_default_embedding_model** | Option<**bool**> |  | [optional]
**transforms** | Option<[**Vec<models::TransformInput>**](TransformInput.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


