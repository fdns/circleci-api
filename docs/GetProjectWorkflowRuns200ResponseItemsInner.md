# GetProjectWorkflowRuns200ResponseItemsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | The unique ID of the workflow. | 
**branch** | **String** | The VCS branch of a Workflow's trigger. | 
**duration** | Option<**i64**> | The duration in seconds of a run. | 
**created_at** | **String** | The date and time the workflow was created. | 
**stopped_at** | Option<**String**> | The date and time the workflow stopped. | 
**credits_used** | **i64** | The number of credits used during execution. Note that Insights is not a real time financial reporting tool and should not be used for credit reporting. | 
**status** | Option<**String**> | Workflow status. | 
**is_approval** | **bool** | Describes if the job is an approval job or not. Approval jobs are intermediary jobs that are created to pause the workflow until approved. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


