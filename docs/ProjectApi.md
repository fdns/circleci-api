# \ProjectApi

All URIs are relative to *https://circleci.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_checkout_key**](ProjectApi.md#create_checkout_key) | **POST** /project/{project-slug}/checkout-key | Create a new checkout key
[**create_env_var**](ProjectApi.md#create_env_var) | **POST** /project/{project-slug}/envvar | Create an environment variable
[**delete_checkout_key**](ProjectApi.md#delete_checkout_key) | **DELETE** /project/{project-slug}/checkout-key/{fingerprint} | Delete a checkout key
[**delete_env_var**](ProjectApi.md#delete_env_var) | **DELETE** /project/{project-slug}/envvar/{name} | Delete an environment variable
[**get_checkout_key**](ProjectApi.md#get_checkout_key) | **GET** /project/{project-slug}/checkout-key/{fingerprint} | Get a checkout key
[**get_env_var**](ProjectApi.md#get_env_var) | **GET** /project/{project-slug}/envvar/{name} | Get a masked environment variable
[**get_project_by_slug**](ProjectApi.md#get_project_by_slug) | **GET** /project/{project-slug} | Get a project
[**list_checkout_keys**](ProjectApi.md#list_checkout_keys) | **GET** /project/{project-slug}/checkout-key | Get all checkout keys
[**list_env_vars**](ProjectApi.md#list_env_vars) | **GET** /project/{project-slug}/envvar | List all environment variables



## create_checkout_key

> crate::models::CheckoutKey create_checkout_key(project_slug, checkout_key_input)
Create a new checkout key

Creates a new checkout key. This API request is only usable with a user API token.                            Please ensure that you have authorized your account with GitHub before creating user keys.                            This is necessary to give CircleCI the permission to create a user key associated with                            your GitHub user account. You can find this page by visiting Project Settings > Checkout SSH Keys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |
**checkout_key_input** | Option<[**CheckoutKeyInput**](CheckoutKeyInput.md)> |  |  |

### Return type

[**crate::models::CheckoutKey**](CheckoutKey.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_env_var

> crate::models::EnvironmentVariable create_env_var(project_slug, environment_variable1)
Create an environment variable

Creates a new environment variable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |
**environment_variable1** | Option<[**EnvironmentVariable1**](EnvironmentVariable1.md)> |  |  |

### Return type

[**crate::models::EnvironmentVariable**](EnvironmentVariable.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_checkout_key

> crate::models::MessageResponse delete_checkout_key(project_slug, fingerprint)
Delete a checkout key

Deletes the checkout key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |
**fingerprint** | **String** | An SSH key fingerprint. | [required] |

### Return type

[**crate::models::MessageResponse**](MessageResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_env_var

> crate::models::MessageResponse delete_env_var(project_slug, name)
Delete an environment variable

Deletes the environment variable named :name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |
**name** | **String** | The name of the environment variable. | [required] |

### Return type

[**crate::models::MessageResponse**](MessageResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_checkout_key

> crate::models::CheckoutKey get_checkout_key(project_slug, fingerprint)
Get a checkout key

Returns an individual checkout key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |
**fingerprint** | **String** | An SSH key fingerprint. | [required] |

### Return type

[**crate::models::CheckoutKey**](CheckoutKey.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_env_var

> crate::models::EnvironmentVariable get_env_var(project_slug, name)
Get a masked environment variable

Returns the masked value of environment variable :name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |
**name** | **String** | The name of the environment variable. | [required] |

### Return type

[**crate::models::EnvironmentVariable**](EnvironmentVariable.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_by_slug

> crate::models::Project get_project_by_slug(project_slug)
Get a project

Retrieves a project by project slug.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |

### Return type

[**crate::models::Project**](Project.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_checkout_keys

> crate::models::CheckoutKeyListResponse list_checkout_keys(project_slug)
Get all checkout keys

Returns a sequence of checkout keys for `:project`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |

### Return type

[**crate::models::CheckoutKeyListResponse**](CheckoutKeyListResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_env_vars

> crate::models::EnvironmentVariableListResponse list_env_vars(project_slug)
List all environment variables

Returns four 'x' characters, in addition to the last four ASCII characters of the value, consistent with the display of environment variable values on the CircleCI website.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |

### Return type

[**crate::models::EnvironmentVariableListResponse**](EnvironmentVariableListResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

