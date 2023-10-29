# CreateScheduleParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the schedule. | 
**timetable** | [**crate::models::ScheduleTimetable**](Schedule_timetable.md) |  | 
**attribution_actor** | **String** | The attribution-actor of the scheduled pipeline. | 
**parameters** | [**::std::collections::HashMap<String, crate::models::ContinuePipelineRequestParametersValue>**](continuePipeline_request_parameters_value.md) | Pipeline parameters represented as key-value pairs. Must contain branch or tag. | 
**description** | Option<**String**> | Description of the schedule. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


