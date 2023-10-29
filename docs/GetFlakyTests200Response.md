# GetFlakyTests200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**flaky_tests** | [**Vec<crate::models::GetFlakyTests200ResponseFlakyTestsInner>**](getFlakyTests_200_response_flaky_tests_inner.md) | A list of all instances of flakes. Note that a test is no longer considered flaky after 2 weeks have passed without a flake. Each flake resets this timer. | 
**total_flaky_tests** | **f64** | A count of unique tests that have failed. If your project has N tests that have flaked multiple times each, this will be equal to N. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


