# \ScheduleApi

All URIs are relative to *https://circleci.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_schedule**](ScheduleApi.md#create_schedule) | **POST** /project/{project-slug}/schedule | Create a schedule
[**delete_schedule_by_id**](ScheduleApi.md#delete_schedule_by_id) | **DELETE** /schedule/{schedule-id} | Delete a schedule
[**get_schedule_by_id**](ScheduleApi.md#get_schedule_by_id) | **GET** /schedule/{schedule-id} | Get a schedule
[**list_schedules_for_project**](ScheduleApi.md#list_schedules_for_project) | **GET** /project/{project-slug}/schedule | Get all schedules
[**update_schedule**](ScheduleApi.md#update_schedule) | **PATCH** /schedule/{schedule-id} | Update a schedule



## create_schedule

> crate::models::Schedule create_schedule(project_slug, create_schedule_parameters)
Create a schedule

Creates a schedule and returns the created schedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |
**create_schedule_parameters** | Option<[**CreateScheduleParameters**](CreateScheduleParameters.md)> |  |  |

### Return type

[**crate::models::Schedule**](Schedule.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_schedule_by_id

> crate::models::MessageResponse delete_schedule_by_id(schedule_id)
Delete a schedule

Deletes the schedule by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule_id** | **uuid::Uuid** | The unique ID of the schedule. | [required] |

### Return type

[**crate::models::MessageResponse**](MessageResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schedule_by_id

> crate::models::Schedule get_schedule_by_id(schedule_id)
Get a schedule

Get a schedule by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule_id** | **uuid::Uuid** | The unique ID of the schedule. | [required] |

### Return type

[**crate::models::Schedule**](Schedule.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_schedules_for_project

> crate::models::ListSchedulesForProject200Response list_schedules_for_project(project_slug, page_token)
Get all schedules

Returns all schedules for this project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |
**page_token** | Option<**String**> | A token to retrieve the next page of results. |  |

### Return type

[**crate::models::ListSchedulesForProject200Response**](listSchedulesForProject_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_schedule

> crate::models::Schedule update_schedule(schedule_id, update_schedule_parameters)
Update a schedule

Updates a schedule and returns the updated schedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule_id** | **uuid::Uuid** | The unique ID of the schedule. | [required] |
**update_schedule_parameters** | Option<[**UpdateScheduleParameters**](UpdateScheduleParameters.md)> |  |  |

### Return type

[**crate::models::Schedule**](Schedule.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

