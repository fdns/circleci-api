# \ContextApi

All URIs are relative to *https://circleci.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_environment_variable_to_context**](ContextApi.md#add_environment_variable_to_context) | **PUT** /context/{context-id}/environment-variable/{env-var-name} | Add or update an environment variable
[**create_context**](ContextApi.md#create_context) | **POST** /context | Create a new context
[**delete_context**](ContextApi.md#delete_context) | **DELETE** /context/{context-id} | Delete a context
[**delete_environment_variable_from_context**](ContextApi.md#delete_environment_variable_from_context) | **DELETE** /context/{context-id}/environment-variable/{env-var-name} | Remove an environment variable
[**get_context**](ContextApi.md#get_context) | **GET** /context/{context-id} | Get a context
[**list_contexts**](ContextApi.md#list_contexts) | **GET** /context | List contexts
[**list_environment_variables_from_context**](ContextApi.md#list_environment_variables_from_context) | **GET** /context/{context-id}/environment-variable | List environment variables



## add_environment_variable_to_context

> crate::models::AddEnvironmentVariableToContext200Response add_environment_variable_to_context(context_id, env_var_name, add_environment_variable_to_context_request)
Add or update an environment variable

Create or update an environment variable within a context. Returns information about the environment variable, not including its value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **uuid::Uuid** | ID of the context (UUID) | [required] |
**env_var_name** | **String** | The name of the environment variable | [required] |
**add_environment_variable_to_context_request** | Option<[**AddEnvironmentVariableToContextRequest**](AddEnvironmentVariableToContextRequest.md)> |  |  |

### Return type

[**crate::models::AddEnvironmentVariableToContext200Response**](addEnvironmentVariableToContext_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_context

> crate::models::Context create_context(create_context_request)
Create a new context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_context_request** | Option<[**CreateContextRequest**](CreateContextRequest.md)> |  |  |

### Return type

[**crate::models::Context**](Context.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_context

> crate::models::MessageResponse delete_context(context_id)
Delete a context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **uuid::Uuid** | ID of the context (UUID) | [required] |

### Return type

[**crate::models::MessageResponse**](MessageResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_environment_variable_from_context

> crate::models::MessageResponse delete_environment_variable_from_context(env_var_name, context_id)
Remove an environment variable

Delete an environment variable from a context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**env_var_name** | **String** | The name of the environment variable | [required] |
**context_id** | **uuid::Uuid** | ID of the context (UUID) | [required] |

### Return type

[**crate::models::MessageResponse**](MessageResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_context

> crate::models::Context get_context(context_id)
Get a context

Returns basic information about a context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **uuid::Uuid** | ID of the context (UUID) | [required] |

### Return type

[**crate::models::Context**](Context.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_contexts

> crate::models::ListContexts200Response list_contexts(owner_id, owner_slug, owner_type, page_token)
List contexts

List all contexts for an owner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner_id** | Option<**uuid::Uuid**> | The unique ID of the owner of the context. Specify either this or owner-slug. |  |
**owner_slug** | Option<**String**> | A string that represents an organization. Specify either this or owner-id. Cannot be used for accounts. |  |
**owner_type** | Option<**String**> | The type of the owner. Defaults to \"organization\". Accounts are only used as context owners in server. |  |
**page_token** | Option<**String**> | A token to retrieve the next page of results. |  |

### Return type

[**crate::models::ListContexts200Response**](listContexts_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_environment_variables_from_context

> crate::models::ListEnvironmentVariablesFromContext200Response list_environment_variables_from_context(context_id, page_token)
List environment variables

List information about environment variables in a context, not including their values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **uuid::Uuid** | ID of the context (UUID) | [required] |
**page_token** | Option<**String**> | A token to retrieve the next page of results. |  |

### Return type

[**crate::models::ListEnvironmentVariablesFromContext200Response**](listEnvironmentVariablesFromContext_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

