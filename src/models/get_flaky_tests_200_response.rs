/*
 * CircleCI API
 *
 * This describes the resources that make up the CircleCI API v2.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetFlakyTests200Response : Flaky tests response

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetFlakyTests200Response {
    /// A list of all instances of flakes. Note that a test is no longer considered flaky after 2 weeks have passed without a flake. Each flake resets this timer.
    #[serde(rename = "flaky-tests")]
    pub flaky_tests: Vec<crate::models::GetFlakyTests200ResponseFlakyTestsInner>,
    /// A count of unique tests that have failed. If your project has N tests that have flaked multiple times each, this will be equal to N.
    #[serde(rename = "total-flaky-tests")]
    pub total_flaky_tests: f64,
}

impl GetFlakyTests200Response {
    /// Flaky tests response
    pub fn new(
        flaky_tests: Vec<crate::models::GetFlakyTests200ResponseFlakyTestsInner>,
        total_flaky_tests: f64,
    ) -> GetFlakyTests200Response {
        GetFlakyTests200Response {
            flaky_tests,
            total_flaky_tests,
        }
    }
}
