# \WebhookApi

All URIs are relative to *https://circleci.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_webhook**](WebhookApi.md#create_webhook) | **POST** /webhook | Create a webhook
[**delete_webhook**](WebhookApi.md#delete_webhook) | **DELETE** /webhook/{webhook-id} | Delete a webhook
[**get_webhook_by_id**](WebhookApi.md#get_webhook_by_id) | **GET** /webhook/{webhook-id} | Get a webhook
[**get_webhooks**](WebhookApi.md#get_webhooks) | **GET** /webhook | List webhooks
[**update_webhook**](WebhookApi.md#update_webhook) | **PUT** /webhook/{webhook-id} | Update a webhook



## create_webhook

> crate::models::Webhook create_webhook(create_webhook_request)
Create a webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_webhook_request** | Option<[**CreateWebhookRequest**](CreateWebhookRequest.md)> |  |  |

### Return type

[**crate::models::Webhook**](Webhook.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhook

> crate::models::MessageResponse delete_webhook(webhook_id)
Delete a webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **uuid::Uuid** | ID of the webhook (UUID) | [required] |

### Return type

[**crate::models::MessageResponse**](MessageResponse.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook_by_id

> crate::models::Webhook get_webhook_by_id(webhook_id)
Get a webhook

Get a webhook by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **uuid::Uuid** | ID of the webhook (UUID) | [required] |

### Return type

[**crate::models::Webhook**](Webhook.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhooks

> crate::models::GetWebhooks200Response get_webhooks(scope_id, scope_type)
List webhooks

Get a list of webhook that match the given scope-type and scope-id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope_id** | **uuid::Uuid** | ID of the scope being used (at the moment, only project ID is supported) | [required] |
**scope_type** | **String** | Type of the scope being used | [required] |

### Return type

[**crate::models::GetWebhooks200Response**](getWebhooks_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook

> crate::models::Webhook update_webhook(webhook_id, update_webhook_request)
Update a webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **uuid::Uuid** | ID of the webhook (UUID) | [required] |
**update_webhook_request** | Option<[**UpdateWebhookRequest**](UpdateWebhookRequest.md)> |  |  |

### Return type

[**crate::models::Webhook**](Webhook.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

