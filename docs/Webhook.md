# Webhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | URL to deliver the webhook to. Note: protocol must be included as well (only https is supported) | 
**verify_tls** | **bool** | Whether to enforce TLS certificate verification when delivering the webhook | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) | The unique ID of the webhook | 
**signing_secret** | **String** | Masked value of the secret used to build an HMAC hash of the payload and passed as a header in the webhook request | 
**updated_at** | **String** | The date and time the webhook was last updated. | 
**name** | **String** | Name of the webhook | 
**created_at** | **String** | The date and time the webhook was created. | 
**scope** | [**crate::models::WebhookScope**](Webhook_scope.md) |  | 
**events** | **Vec<String>** | Events that will trigger the webhook | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


