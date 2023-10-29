# CreateWebhookRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the webhook | 
**events** | **Vec<String>** | Events that will trigger the webhook | 
**url** | **String** | URL to deliver the webhook to. Note: protocol must be included as well (only https is supported) | 
**verify_tls** | **bool** | Whether to enforce TLS certificate verification when delivering the webhook | 
**signing_secret** | **String** | Secret used to build an HMAC hash of the payload and passed as a header in the webhook request | 
**scope** | [**crate::models::CreateWebhookRequestScope**](createWebhook_request_scope.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


