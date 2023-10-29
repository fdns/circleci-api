# GetProjectWorkflowTestMetrics200ResponseMostFailedTestsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**p95_duration** | Option<**f64**> | The 95th percentile duration, in seconds, among a group of test runs. | 
**total_runs** | **i64** | The total number of times the test was run. | 
**classname** | Option<**String**> | The class the test belongs to. | 
**failed_runs** | **i64** | The number of times the test failed | 
**flaky** | **bool** | Whether the test is flaky. | 
**source** | Option<**String**> | The source of the test. | 
**file** | Option<**String**> | The file the test belongs to. | 
**job_name** | **String** | The name of the job. | 
**test_name** | **String** | The name of the test. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


