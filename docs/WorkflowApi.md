# \WorkflowApi

All URIs are relative to *https://circleci.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**approve_pending_approval_job_by_id**](WorkflowApi.md#approve_pending_approval_job_by_id) | **POST** /workflow/{id}/approve/{approval_request_id} | Approve a job
[**cancel_workflow**](WorkflowApi.md#cancel_workflow) | **POST** /workflow/{id}/cancel | Cancel a workflow
[**get_workflow_by_id**](WorkflowApi.md#get_workflow_by_id) | **GET** /workflow/{id} | Get a workflow
[**list_workflow_jobs**](WorkflowApi.md#list_workflow_jobs) | **GET** /workflow/{id}/job | Get a workflow's jobs
[**rerun_workflow**](WorkflowApi.md#rerun_workflow) | **POST** /workflow/{id}/rerun | Rerun a workflow



## approve_pending_approval_job_by_id

> crate::models::MessageResponse approve_pending_approval_job_by_id(approval_request_id, id)
Approve a job

Approves a pending approval job in a workflow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**approval_request_id** | **uuid::Uuid** | The ID of the job being approved. | [required] |
**id** | **uuid::Uuid** | The unique ID of the workflow. | [required] |

### Return type

[**crate::models::MessageResponse**](MessageResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_workflow

> crate::models::MessageResponse cancel_workflow(id)
Cancel a workflow

Cancels a running workflow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The unique ID of the workflow. | [required] |

### Return type

[**crate::models::MessageResponse**](MessageResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow_by_id

> crate::models::Workflow get_workflow_by_id(id)
Get a workflow

Returns summary fields of a workflow by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The unique ID of the workflow. | [required] |

### Return type

[**crate::models::Workflow**](Workflow.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_workflow_jobs

> crate::models::WorkflowJobListResponse list_workflow_jobs(id)
Get a workflow's jobs

Returns a sequence of jobs for a workflow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The unique ID of the workflow. | [required] |

### Return type

[**crate::models::WorkflowJobListResponse**](WorkflowJobListResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rerun_workflow

> crate::models::RerunWorkflow202Response rerun_workflow(id, rerun_workflow_parameters)
Rerun a workflow

Reruns a workflow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The unique ID of the workflow. | [required] |
**rerun_workflow_parameters** | Option<[**RerunWorkflowParameters**](RerunWorkflowParameters.md)> |  |  |

### Return type

[**crate::models::RerunWorkflow202Response**](rerunWorkflow_202_response.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

