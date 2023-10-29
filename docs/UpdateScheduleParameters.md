# UpdateScheduleParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Description of the schedule. | [optional]
**name** | Option<**String**> | Name of the schedule. | [optional]
**timetable** | Option<[**crate::models::UpdateScheduleParametersTimetable**](UpdateScheduleParameters_timetable.md)> |  | [optional]
**attribution_actor** | Option<**String**> | The attribution-actor of the scheduled pipeline. | [optional]
**parameters** | Option<[**::std::collections::HashMap<String, crate::models::ContinuePipelineRequestParametersValue>**](continuePipeline_request_parameters_value.md)> | Pipeline parameters represented as key-value pairs. Must contain branch or tag. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


