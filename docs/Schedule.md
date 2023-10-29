# Schedule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | The unique ID of the schedule. | 
**timetable** | [**crate::models::ScheduleTimetable**](Schedule_timetable.md) |  | 
**updated_at** | **String** | The date and time the pipeline was last updated. | 
**name** | **String** | Name of the schedule. | 
**created_at** | **String** | The date and time the pipeline was created. | 
**project_slug** | **String** | The project-slug for the schedule | 
**parameters** | [**::std::collections::HashMap<String, crate::models::ContinuePipelineRequestParametersValue>**](continuePipeline_request_parameters_value.md) | Pipeline parameters represented as key-value pairs. Must contain branch or tag. | 
**actor** | [**crate::models::User1**](User_1.md) |  | 
**description** | Option<**String**> | Description of the schedule. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


