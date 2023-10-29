# GetWorkflowSummary200ResponseMetrics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_runs** | **i64** | The total number of runs, including runs that are still on-hold or running. | 
**successful_runs** | **i64** | The number of successful runs. | 
**mttr** | Option<**i64**> | The mean time to recovery (mean time between failures and their next success) in seconds. | 
**total_credits_used** | Option<**i64**> | The total credits consumed by the workflow in the aggregation window. Note that Insights is not a real time financial reporting tool and should not be used for credit reporting. | 
**failed_runs** | **i64** | The number of failed runs. | 
**success_rate** | **f32** |  | 
**completed_runs** | Option<**i64**> | The number of runs that ran to completion within the aggregation window | 
**window_start** | **String** | The timestamp of the first build within the requested reporting window. | 
**duration_metrics** | [**crate::models::GetProjectWorkflowMetrics200ResponseItemsInnerMetricsDurationMetrics**](getProjectWorkflowMetrics_200_response_items_inner_metrics_duration_metrics.md) |  | 
**window_end** | **String** | The timestamp of the last build within the requested reporting window. | 
**throughput** | **f32** | The average number of runs per day. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


