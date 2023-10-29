# Pipeline

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | The unique ID of the pipeline. | 
**errors** | [**Vec<crate::models::PipelineErrorsInner>**](Pipeline_errors_inner.md) | A sequence of errors that have occurred within the pipeline. | 
**project_slug** | **String** | The project-slug for the pipeline. | 
**updated_at** | Option<**String**> | The date and time the pipeline was last updated. | [optional]
**number** | **i64** | The number of the pipeline. | 
**trigger_parameters** | Option<[**::std::collections::HashMap<String, crate::models::PipelineTriggerParametersValue>**](Pipeline_trigger_parameters_value.md)> |  | [optional]
**state** | **String** | The current state of the pipeline. | 
**created_at** | **String** | The date and time the pipeline was created. | 
**trigger** | [**crate::models::PipelineTrigger**](Pipeline_trigger.md) |  | 
**vcs** | Option<[**crate::models::PipelineVcs**](Pipeline_vcs.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


