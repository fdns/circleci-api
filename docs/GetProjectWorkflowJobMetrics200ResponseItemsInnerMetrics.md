# GetProjectWorkflowJobMetrics200ResponseItemsInnerMetrics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_runs** | **i64** | The total number of runs, including runs that are still on-hold or running. | 
**failed_runs** | **i64** | The number of failed runs. | 
**successful_runs** | **i64** | The number of successful runs. | 
**duration_metrics** | [**crate::models::GetProjectWorkflowJobMetrics200ResponseItemsInnerMetricsDurationMetrics**](getProjectWorkflowJobMetrics_200_response_items_inner_metrics_duration_metrics.md) |  | 
**success_rate** | **f32** |  | 
**total_credits_used** | **i64** | The total credits consumed by the job in the aggregation window. Note that Insights is not a real time financial reporting tool and should not be used for credit reporting. | 
**throughput** | **f32** | The average number of runs per day. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


