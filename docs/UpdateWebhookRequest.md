# UpdateWebhookRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the webhook | [optional]
**events** | Option<**Vec<String>**> | Events that will trigger the webhook | [optional]
**url** | Option<**String**> | URL to deliver the webhook to. Note: protocol must be included as well (only https is supported) | [optional]
**signing_secret** | Option<**String**> | Secret used to build an HMAC hash of the payload and passed as a header in the webhook request | [optional]
**verify_tls** | Option<**bool**> | Whether to enforce TLS certificate verification when delivering the webhook | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


