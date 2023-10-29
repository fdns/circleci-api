# \PolicyManagementApi

All URIs are relative to *https://circleci.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_policy_bundle**](PolicyManagementApi.md#create_policy_bundle) | **POST** /owner/{ownerID}/context/{context}/policy-bundle | Creates policy bundle for the context
[**get_decision_log**](PolicyManagementApi.md#get_decision_log) | **GET** /owner/{ownerID}/context/{context}/decision/{decisionID} | Retrieves the owner's decision audit log by given decisionID
[**get_decision_log_policy_bundle**](PolicyManagementApi.md#get_decision_log_policy_bundle) | **GET** /owner/{ownerID}/context/{context}/decision/{decisionID}/policy-bundle | Retrieves Policy Bundle for a given decision log ID
[**get_decision_logs**](PolicyManagementApi.md#get_decision_logs) | **GET** /owner/{ownerID}/context/{context}/decision | Retrieves the owner's decision audit logs.
[**get_decision_settings**](PolicyManagementApi.md#get_decision_settings) | **GET** /owner/{ownerID}/context/{context}/decision/settings | Get the decision settings
[**get_policy_bundle**](PolicyManagementApi.md#get_policy_bundle) | **GET** /owner/{ownerID}/context/{context}/policy-bundle | Retrieves Policy Bundle
[**get_policy_document**](PolicyManagementApi.md#get_policy_document) | **GET** /owner/{ownerID}/context/{context}/policy-bundle/{policyName} | Retrieves a policy document
[**make_decision**](PolicyManagementApi.md#make_decision) | **POST** /owner/{ownerID}/context/{context}/decision | Makes a decision
[**set_decision_settings**](PolicyManagementApi.md#set_decision_settings) | **PATCH** /owner/{ownerID}/context/{context}/decision/settings | Set the decision settings



## create_policy_bundle

> crate::models::BundleDiff create_policy_bundle(owner_id, context, dry, bundle_payload)
Creates policy bundle for the context

This endpoint replaces the current policy bundle with the provided policy bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner_id** | **String** |  | [required] |
**context** | **String** |  | [required] |
**dry** | Option<**bool**> |  |  |
**bundle_payload** | Option<[**BundlePayload**](BundlePayload.md)> |  |  |

### Return type

[**crate::models::BundleDiff**](BundleDiff.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_decision_log

> crate::models::DecisionLog get_decision_log(owner_id, context, decision_id)
Retrieves the owner's decision audit log by given decisionID

This endpoint will retrieve a decision for a given decision log ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner_id** | **String** |  | [required] |
**context** | **String** |  | [required] |
**decision_id** | **String** |  | [required] |

### Return type

[**crate::models::DecisionLog**](DecisionLog.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_decision_log_policy_bundle

> ::std::collections::HashMap<String, Vec<crate::models::Policy>> get_decision_log_policy_bundle(owner_id, context, decision_id)
Retrieves Policy Bundle for a given decision log ID

This endpoint will retrieve a policy bundle for a given decision log ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner_id** | **String** |  | [required] |
**context** | **String** |  | [required] |
**decision_id** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, Vec<crate::models::Policy>>**](array.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_decision_logs

> Vec<crate::models::DecisionLog> get_decision_logs(owner_id, context, status, after, before, branch, project_id, build_number, offset)
Retrieves the owner's decision audit logs.

This endpoint will return a list of decision audit logs that were made using this owner's policies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner_id** | **String** |  | [required] |
**context** | **String** |  | [required] |
**status** | Option<**String**> | Return decisions matching this decision status. |  |
**after** | Option<**String**> | Return decisions made after this date. |  |
**before** | Option<**String**> | Return decisions made before this date. |  |
**branch** | Option<**String**> | Return decisions made on this branch. |  |
**project_id** | Option<**String**> | Return decisions made for this project. |  |
**build_number** | Option<**String**> | Return decisions made for this build number. |  |
**offset** | Option<**i32**> | Sets the offset when retrieving the decisions, for paging. |  |

### Return type

[**Vec<crate::models::DecisionLog>**](DecisionLog.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_decision_settings

> crate::models::DecisionSettings get_decision_settings(owner_id, context)
Get the decision settings

This endpoint retrieves the current decision settings (eg enable/disable policy evaluation)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner_id** | **String** |  | [required] |
**context** | **String** |  | [required] |

### Return type

[**crate::models::DecisionSettings**](DecisionSettings.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_policy_bundle

> ::std::collections::HashMap<String, Vec<crate::models::Policy>> get_policy_bundle(owner_id, context)
Retrieves Policy Bundle

This endpoint will retrieve a policy bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner_id** | **String** |  | [required] |
**context** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, Vec<crate::models::Policy>>**](array.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_policy_document

> crate::models::Policy get_policy_document(owner_id, context, policy_name)
Retrieves a policy document

This endpoint will retrieve a policy document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner_id** | **String** |  | [required] |
**context** | **String** |  | [required] |
**policy_name** | **String** | the policy name set by the rego policy_name rule | [required] |

### Return type

[**crate::models::Policy**](Policy.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## make_decision

> crate::models::Decision make_decision(owner_id, context, make_decision_request)
Makes a decision

This endpoint will evaluate input data (config+metadata) against owner's stored policies and return a decision.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner_id** | **String** |  | [required] |
**context** | **String** |  | [required] |
**make_decision_request** | Option<[**MakeDecisionRequest**](MakeDecisionRequest.md)> |  |  |

### Return type

[**crate::models::Decision**](Decision.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_decision_settings

> crate::models::DecisionSettings set_decision_settings(owner_id, context, decision_settings)
Set the decision settings

This endpoint allows modifying decision settings (eg enable/disable policy evaluation)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner_id** | **String** |  | [required] |
**context** | **String** |  | [required] |
**decision_settings** | Option<[**DecisionSettings**](DecisionSettings.md)> |  |  |

### Return type

[**crate::models::DecisionSettings**](DecisionSettings.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

