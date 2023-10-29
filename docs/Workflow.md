# Workflow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pipeline_id** | [**uuid::Uuid**](uuid::Uuid.md) | The ID of the pipeline this workflow belongs to. | 
**canceled_by** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**id** | [**uuid::Uuid**](uuid::Uuid.md) | The unique ID of the workflow. | 
**name** | **String** | The name of the workflow. | 
**project_slug** | **String** | The project-slug for the pipeline this workflow belongs to. | 
**errored_by** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**tag** | Option<**String**> | Tag used for the workflow | [optional]
**status** | **String** | The current status of the workflow. | 
**started_by** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**pipeline_number** | **i64** | The number of the pipeline this workflow belongs to. | 
**created_at** | **String** | The date and time the workflow was created. | 
**stopped_at** | Option<**String**> | The date and time the workflow stopped. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


