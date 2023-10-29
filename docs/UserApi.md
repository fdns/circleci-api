# \UserApi

All URIs are relative to *https://circleci.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_collaborations**](UserApi.md#get_collaborations) | **GET** /me/collaborations | Collaborations
[**get_current_user**](UserApi.md#get_current_user) | **GET** /me | User Information
[**get_user**](UserApi.md#get_user) | **GET** /user/{id} | User Information



## get_collaborations

> Vec<crate::models::Collaboration> get_collaborations()
Collaborations

Provides the set of organizations of which a user is a member or a collaborator.  The set of organizations that a user can collaborate on is composed of:  * Organizations that the current user belongs to across VCS types (e.g. BitBucket, GitHub) * The parent organization of repository that the user can collaborate on, but is not necessarily a member of * The organization of the current user's account

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Collaboration>**](Collaboration.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user

> crate::models::User get_current_user()
User Information

Provides information about the user that is currently signed in.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::User**](User.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> crate::models::User get_user(id)
User Information

Provides information about the user with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The unique ID of the user. | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

