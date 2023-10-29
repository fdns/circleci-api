# GetFlakyTests200ResponseFlakyTestsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**time_wasted** | Option<**i64**> |  | [optional]
**workflow_created_at** | Option<[**serde_json::Value**](.md)> | The date and time when workflow was created. | 
**workflow_id** | Option<[**serde_json::Value**](.md)> | The ID of the workflow associated with the provided test counts | 
**classname** | Option<**String**> | The class the test belongs to. | 
**pipeline_number** | **i64** | The number of the pipeline. | 
**workflow_name** | **String** | The name of the workflow. | 
**test_name** | **String** | The name of the test. | 
**job_name** | **String** | The name of the job. | 
**job_number** | **i64** | The number of the job. | 
**times_flaked** | **i64** | The number of times the test flaked. | 
**source** | Option<**String**> | The source of the test. | 
**file** | Option<**String**> | The file the test belongs to. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


