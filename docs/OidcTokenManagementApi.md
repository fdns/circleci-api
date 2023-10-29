# \OidcTokenManagementApi

All URIs are relative to *https://circleci.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_org_claims**](OidcTokenManagementApi.md#delete_org_claims) | **DELETE** /org/{orgID}/oidc-custom-claims | Delete org-level claims
[**delete_project_claims**](OidcTokenManagementApi.md#delete_project_claims) | **DELETE** /org/{orgID}/project/{projectID}/oidc-custom-claims | Delete project-level claims
[**get_org_claims**](OidcTokenManagementApi.md#get_org_claims) | **GET** /org/{orgID}/oidc-custom-claims | Get org-level claims
[**get_project_claims**](OidcTokenManagementApi.md#get_project_claims) | **GET** /org/{orgID}/project/{projectID}/oidc-custom-claims | Get project-level claims
[**patch_org_claims**](OidcTokenManagementApi.md#patch_org_claims) | **PATCH** /org/{orgID}/oidc-custom-claims | Patch org-level claims
[**patch_project_claims**](OidcTokenManagementApi.md#patch_project_claims) | **PATCH** /org/{orgID}/project/{projectID}/oidc-custom-claims | Patch project-level claims



## delete_org_claims

> crate::models::ClaimResponse delete_org_claims(org_id, claims)
Delete org-level claims

Deletes org-level custom claims of OIDC identity tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** |  | [required] |
**claims** | **String** | comma separated list of claims to delete. Valid values are \"audience\" and \"ttl\". | [required] |

### Return type

[**crate::models::ClaimResponse**](ClaimResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project_claims

> crate::models::ClaimResponse delete_project_claims(org_id, project_id, claims)
Delete project-level claims

Deletes project-level custom claims of OIDC identity tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** |  | [required] |
**project_id** | **uuid::Uuid** |  | [required] |
**claims** | **String** | comma separated list of claims to delete. Valid values are \"audience\" and \"ttl\". | [required] |

### Return type

[**crate::models::ClaimResponse**](ClaimResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_org_claims

> crate::models::ClaimResponse get_org_claims(org_id)
Get org-level claims

Fetches org-level custom claims of OIDC identity tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::ClaimResponse**](ClaimResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_claims

> crate::models::ClaimResponse get_project_claims(org_id, project_id)
Get project-level claims

Fetches project-level custom claims of OIDC identity tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** |  | [required] |
**project_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::ClaimResponse**](ClaimResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_org_claims

> crate::models::ClaimResponse patch_org_claims(org_id, patch_claims_request)
Patch org-level claims

Creates/Updates org-level custom claims of OIDC identity tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** |  | [required] |
**patch_claims_request** | Option<[**PatchClaimsRequest**](PatchClaimsRequest.md)> |  |  |

### Return type

[**crate::models::ClaimResponse**](ClaimResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_project_claims

> crate::models::ClaimResponse patch_project_claims(org_id, project_id, patch_claims_request)
Patch project-level claims

Creates/Updates project-level custom claims of OIDC identity tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** |  | [required] |
**project_id** | **uuid::Uuid** |  | [required] |
**patch_claims_request** | Option<[**PatchClaimsRequest**](PatchClaimsRequest.md)> |  |  |

### Return type

[**crate::models::ClaimResponse**](ClaimResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

