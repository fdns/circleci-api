# GetProjectWorkflowsPageData200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**org_id** | Option<[**serde_json::Value**](.md)> | The unique ID of the organization | [optional]
**project_id** | Option<[**serde_json::Value**](.md)> | The unique ID of the project | [optional]
**project_data** | Option<[**crate::models::GetProjectWorkflowsPageData200ResponseProjectData**](getProjectWorkflowsPageData_200_response_project_data.md)> |  | [optional]
**project_workflow_data** | Option<[**Vec<crate::models::GetProjectWorkflowsPageData200ResponseProjectWorkflowDataInner>**](getProjectWorkflowsPageData_200_response_project_workflow_data_inner.md)> | A list of metrics and trends data for workflows for a given project. | [optional]
**project_workflow_branch_data** | Option<[**Vec<crate::models::GetProjectWorkflowsPageData200ResponseProjectWorkflowBranchDataInner>**](getProjectWorkflowsPageData_200_response_project_workflow_branch_data_inner.md)> | A list of metrics and trends data for branches for a given project. | [optional]
**all_branches** | Option<**Vec<String>**> | A list of all the branches for a given project. | [optional]
**all_workflows** | Option<**Vec<String>**> | A list of all the workflows for a given project. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


