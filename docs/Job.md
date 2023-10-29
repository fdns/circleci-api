# Job

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**canceled_by** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The unique ID of the user. | [optional]
**dependencies** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | A sequence of the unique job IDs for the jobs that this job depends upon in the workflow. | 
**job_number** | Option<**i64**> | The number of the job. | [optional]
**id** | [**uuid::Uuid**](uuid::Uuid.md) | The unique ID of the job. | 
**started_at** | **String** | The date and time the job started. | 
**name** | **String** | The name of the job. | 
**approved_by** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The unique ID of the user. | [optional]
**project_slug** | **String** | The project-slug for the job. | 
**status** | **String** | The current status of the job. | 
**r#type** | **String** | The type of job. | 
**stopped_at** | Option<**String**> | The time when the job stopped. | [optional]
**approval_request_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The unique ID of the job. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


