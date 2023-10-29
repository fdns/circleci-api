# RerunWorkflowParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enable_ssh** | Option<**bool**> | Whether to enable SSH access for the triggering user on the newly-rerun job. Requires the jobs parameter to be used and so is mutually exclusive with the from_failed parameter. | [optional]
**from_failed** | Option<**bool**> | Whether to rerun the workflow from the failed job. Mutually exclusive with the jobs parameter. | [optional]
**jobs** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | A list of job IDs to rerun. | [optional]
**sparse_tree** | Option<**bool**> | Completes rerun using sparse trees logic, an optimization for workflows that have disconnected subgraphs. Requires jobs parameter and so is mutually exclusive with the from_failed parameter. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


