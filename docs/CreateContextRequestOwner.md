# CreateContextRequestOwner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | The unique ID of the owner of the context. Specify either this or slug. | 
**r#type** | Option<**String**> | The type of the owner. Defaults to \"organization\". Accounts are only used as context owners in server. | [optional]
**slug** | **String** | A string that represents an organization. Specify either this or id. Cannot be used for accounts. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


