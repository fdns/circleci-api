# ScheduleTimetable

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**per_hour** | **i32** | Number of times a schedule triggers per hour, value must be between 1 and 60 | 
**hours_of_day** | **Vec<i32>** | Hours in a day in which the schedule triggers. | 
**days_of_week** | **Vec<String>** | Days in a week in which the schedule triggers. | 
**days_of_month** | **Vec<i32>** | Days in a month in which the schedule triggers. This is mutually exclusive with days in a week. | 
**months** | Option<**Vec<String>**> | Months in which the schedule triggers. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


