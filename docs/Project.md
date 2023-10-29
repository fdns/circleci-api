# Project

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | 
**name** | **String** | The name of the project | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**organization_name** | **String** | The name of the organization the project belongs to | 
**organization_slug** | **String** | The slug of the organization the project belongs to | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | The id of the organization the project belongs to | 
**vcs_info** | [**crate::models::ProjectVcsInfo**](Project_vcs_info.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


