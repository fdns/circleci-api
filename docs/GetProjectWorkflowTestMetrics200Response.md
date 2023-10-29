# GetProjectWorkflowTestMetrics200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**average_test_count** | **i64** | The average number of tests executed per run | 
**most_failed_tests** | [**Vec<crate::models::GetProjectWorkflowTestMetrics200ResponseMostFailedTestsInner>**](getProjectWorkflowTestMetrics_200_response_most_failed_tests_inner.md) | Metrics for the most frequently failing tests | 
**most_failed_tests_extra** | **i64** | The number of tests with the same success rate being omitted from most_failed_tests | 
**slowest_tests** | [**Vec<crate::models::GetProjectWorkflowTestMetrics200ResponseMostFailedTestsInner>**](getProjectWorkflowTestMetrics_200_response_most_failed_tests_inner.md) | Metrics for the slowest running tests | 
**slowest_tests_extra** | **i64** | The number of tests with the same duration rate being omitted from slowest_tests | 
**total_test_runs** | **i64** | The total number of test runs | 
**test_runs** | [**Vec<crate::models::GetProjectWorkflowTestMetrics200ResponseTestRunsInner>**](getProjectWorkflowTestMetrics_200_response_test_runs_inner.md) | Test counts grouped by pipeline number and workflow id | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


