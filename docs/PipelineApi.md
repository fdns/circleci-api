# \PipelineApi

All URIs are relative to *https://circleci.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**continue_pipeline**](PipelineApi.md#continue_pipeline) | **POST** /pipeline/continue | Continue a pipeline
[**get_pipeline_by_id**](PipelineApi.md#get_pipeline_by_id) | **GET** /pipeline/{pipeline-id} | Get a pipeline by ID
[**get_pipeline_by_number**](PipelineApi.md#get_pipeline_by_number) | **GET** /project/{project-slug}/pipeline/{pipeline-number} | Get a pipeline by pipeline number
[**get_pipeline_config_by_id**](PipelineApi.md#get_pipeline_config_by_id) | **GET** /pipeline/{pipeline-id}/config | Get a pipeline's configuration
[**list_my_pipelines**](PipelineApi.md#list_my_pipelines) | **GET** /project/{project-slug}/pipeline/mine | Get your pipelines
[**list_pipelines**](PipelineApi.md#list_pipelines) | **GET** /pipeline | Get a list of pipelines
[**list_pipelines_for_project**](PipelineApi.md#list_pipelines_for_project) | **GET** /project/{project-slug}/pipeline | Get all pipelines
[**list_workflows_by_pipeline_id**](PipelineApi.md#list_workflows_by_pipeline_id) | **GET** /pipeline/{pipeline-id}/workflow | Get a pipeline's workflows
[**trigger_pipeline**](PipelineApi.md#trigger_pipeline) | **POST** /project/{project-slug}/pipeline | Trigger a new pipeline



## continue_pipeline

> crate::models::MessageResponse continue_pipeline(continue_pipeline_request)
Continue a pipeline

Continue a pipeline from the setup phase.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**continue_pipeline_request** | Option<[**ContinuePipelineRequest**](ContinuePipelineRequest.md)> |  |  |

### Return type

[**crate::models::MessageResponse**](MessageResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_by_id

> crate::models::Pipeline get_pipeline_by_id(pipeline_id)
Get a pipeline by ID

Returns a pipeline by the pipeline ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **uuid::Uuid** | The unique ID of the pipeline. | [required] |

### Return type

[**crate::models::Pipeline**](Pipeline.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_by_number

> crate::models::Pipeline get_pipeline_by_number(project_slug, pipeline_number)
Get a pipeline by pipeline number

Returns a pipeline by the pipeline number.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |
**pipeline_number** | [**serde_json::Value**](.md) | The number of the pipeline. | [required] |

### Return type

[**crate::models::Pipeline**](Pipeline.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_config_by_id

> crate::models::PipelineConfig get_pipeline_config_by_id(pipeline_id)
Get a pipeline's configuration

Returns a pipeline's configuration by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **uuid::Uuid** | The unique ID of the pipeline. | [required] |

### Return type

[**crate::models::PipelineConfig**](PipelineConfig.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_my_pipelines

> crate::models::PipelineListResponse list_my_pipelines(project_slug, page_token)
Get your pipelines

Returns a sequence of all pipelines for this project triggered by the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |
**page_token** | Option<**String**> | A token to retrieve the next page of results. |  |

### Return type

[**crate::models::PipelineListResponse**](PipelineListResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_pipelines

> crate::models::PipelineListResponse list_pipelines(org_slug, page_token, mine)
Get a list of pipelines

Returns all pipelines for the most recently built projects (max 250) you follow in an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | Option<**String**> | Org slug in the form `vcs-slug/org-name` |  |
**page_token** | Option<**String**> | A token to retrieve the next page of results. |  |
**mine** | Option<**bool**> | Only include entries created by your user. |  |

### Return type

[**crate::models::PipelineListResponse**](PipelineListResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_pipelines_for_project

> crate::models::PipelineListResponse list_pipelines_for_project(project_slug, branch, page_token)
Get all pipelines

Returns all pipelines for this project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |
**branch** | Option<**String**> | The name of a vcs branch. |  |
**page_token** | Option<**String**> | A token to retrieve the next page of results. |  |

### Return type

[**crate::models::PipelineListResponse**](PipelineListResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_workflows_by_pipeline_id

> crate::models::WorkflowListResponse list_workflows_by_pipeline_id(pipeline_id, page_token)
Get a pipeline's workflows

Returns a paginated list of workflows by pipeline ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **uuid::Uuid** | The unique ID of the pipeline. | [required] |
**page_token** | Option<**String**> | A token to retrieve the next page of results. |  |

### Return type

[**crate::models::WorkflowListResponse**](WorkflowListResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_pipeline

> crate::models::PipelineCreation trigger_pipeline(project_slug, trigger_pipeline_parameters)
Trigger a new pipeline

Triggers a new pipeline on the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |
**trigger_pipeline_parameters** | Option<[**TriggerPipelineParameters**](TriggerPipelineParameters.md)> |  |  |

### Return type

[**crate::models::PipelineCreation**](PipelineCreation.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

