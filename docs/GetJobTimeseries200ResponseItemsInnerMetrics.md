# GetJobTimeseries200ResponseItemsInnerMetrics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_runs** | **i64** | The total number of runs, including runs that are still on-hold or running. | 
**failed_runs** | **i64** | The number of failed runs. | 
**successful_runs** | **i64** | The number of successful runs. | 
**throughput** | **f32** | The average number of runs per day. | 
**median_credits_used** | **i64** | The median credits consumed over the current timeseries interval. | 
**total_credits_used** | **i64** | The total credits consumed over the current timeseries interval. | 
**duration_metrics** | [**crate::models::GetJobTimeseries200ResponseItemsInnerMetricsDurationMetrics**](getJobTimeseries_200_response_items_inner_metrics_duration_metrics.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


