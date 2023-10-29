# GetProjectWorkflowMetrics200ResponseItemsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the workflow. | 
**metrics** | [**crate::models::GetProjectWorkflowMetrics200ResponseItemsInnerMetrics**](getProjectWorkflowMetrics_200_response_items_inner_metrics.md) |  | 
**window_start** | **String** | The timestamp of the first build within the requested reporting window. | 
**window_end** | **String** | The timestamp of the last build within the requested reporting window. | 
**project_id** | Option<[**serde_json::Value**](.md)> | The unique ID of the project | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


