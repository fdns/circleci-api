/*
 * CircleCI API
 *
 * This describes the resources that make up the CircleCI API v2.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateScheduleParameters : The parameters for a create schedule request

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateScheduleParameters {
    /// Name of the schedule.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "timetable")]
    pub timetable: Box<crate::models::ScheduleTimetable>,
    /// The attribution-actor of the scheduled pipeline.
    #[serde(rename = "attribution-actor")]
    pub attribution_actor: AttributionActor,
    /// Pipeline parameters represented as key-value pairs. Must contain branch or tag.
    #[serde(rename = "parameters")]
    pub parameters:
        ::std::collections::HashMap<String, crate::models::ContinuePipelineRequestParametersValue>,
    /// Description of the schedule.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
}

impl CreateScheduleParameters {
    /// The parameters for a create schedule request
    pub fn new(
        name: String,
        timetable: crate::models::ScheduleTimetable,
        attribution_actor: AttributionActor,
        parameters: ::std::collections::HashMap<
            String,
            crate::models::ContinuePipelineRequestParametersValue,
        >,
    ) -> CreateScheduleParameters {
        CreateScheduleParameters {
            name,
            timetable: Box::new(timetable),
            attribution_actor,
            parameters,
            description: None,
        }
    }
}

/// The attribution-actor of the scheduled pipeline.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AttributionActor {
    #[serde(rename = "current")]
    Current,
    #[serde(rename = "system")]
    System,
}

impl Default for AttributionActor {
    fn default() -> AttributionActor {
        Self::Current
    }
}
