# \JobApi

All URIs are relative to *https://circleci.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_job**](JobApi.md#cancel_job) | **POST** /project/{project-slug}/job/{job-number}/cancel | Cancel job
[**get_job_artifacts**](JobApi.md#get_job_artifacts) | **GET** /project/{project-slug}/{job-number}/artifacts | Get a job's artifacts
[**get_job_details**](JobApi.md#get_job_details) | **GET** /project/{project-slug}/job/{job-number} | Get job details
[**get_tests**](JobApi.md#get_tests) | **GET** /project/{project-slug}/{job-number}/tests | Get test metadata



## cancel_job

> crate::models::MessageResponse cancel_job(job_number, project_slug)
Cancel job

Cancel job with a given job number.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_number** | [**serde_json::Value**](.md) | The number of the job. | [required] |
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |

### Return type

[**crate::models::MessageResponse**](MessageResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_artifacts

> crate::models::ArtifactListResponse get_job_artifacts(job_number, project_slug)
Get a job's artifacts

Returns a job's artifacts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_number** | [**serde_json::Value**](.md) | The number of the job. | [required] |
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |

### Return type

[**crate::models::ArtifactListResponse**](ArtifactListResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_details

> crate::models::JobDetails get_job_details(job_number, project_slug)
Get job details

Returns job details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_number** | [**serde_json::Value**](.md) | The number of the job. | [required] |
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |

### Return type

[**crate::models::JobDetails**](Job_Details.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tests

> crate::models::TestsResponse get_tests(job_number, project_slug)
Get test metadata

Get test metadata for a build. In the rare case where there is more than 250MB of test data on the job, no results will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_number** | [**serde_json::Value**](.md) | The number of the job. | [required] |
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |

### Return type

[**crate::models::TestsResponse**](TestsResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

