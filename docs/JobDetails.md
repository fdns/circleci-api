# JobDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**web_url** | **String** | URL of the job in CircleCI Web UI. | 
**project** | [**crate::models::JobDetailsProject**](Job_Details_project.md) |  | 
**parallel_runs** | [**Vec<crate::models::JobDetailsParallelRunsInner>**](Job_Details_parallel_runs_inner.md) | Info about parallels runs and their status. | 
**started_at** | **String** | The date and time the job started. | 
**latest_workflow** | [**crate::models::JobDetailsLatestWorkflow**](Job_Details_latest_workflow.md) |  | 
**name** | **String** | The name of the job. | 
**executor** | [**crate::models::JobDetailsExecutor**](Job_Details_executor.md) |  | 
**parallelism** | **i64** | A number of parallel runs the job has. | 
**status** | **String** | The current status of the job. | 
**number** | **i64** | The number of the job. | 
**pipeline** | [**crate::models::JobDetailsPipeline**](Job_Details_pipeline.md) |  | 
**duration** | Option<**i64**> | Duration of a job in milliseconds. | 
**created_at** | **String** | The time when the job was created. | 
**messages** | [**Vec<crate::models::JobDetailsMessagesInner>**](Job_Details_messages_inner.md) | Messages from CircleCI execution platform. | 
**contexts** | [**Vec<crate::models::JobDetailsContextsInner>**](Job_Details_contexts_inner.md) | List of contexts used by the job. | 
**organization** | [**crate::models::JobDetailsOrganization**](Job_Details_organization.md) |  | 
**queued_at** | **String** | The time when the job was placed in a queue. | 
**stopped_at** | Option<**String**> | The time when the job stopped. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


