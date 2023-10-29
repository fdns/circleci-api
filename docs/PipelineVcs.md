# PipelineVcs

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider_name** | **String** | Name of the VCS provider (e.g. GitHub, Bitbucket). | 
**target_repository_url** | **String** | URL for the repository the trigger targets (i.e. the repository where the PR will be merged). For fork-PR pipelines, this is the URL to the parent repo. For other pipelines, the `origin_` and `target_repository_url`s will be the same. | 
**branch** | Option<**String**> | The branch where the pipeline ran. The HEAD commit on this branch was used for the pipeline. Note that `branch` and `tag` are mutually exclusive. To trigger a pipeline for a PR by number use `pull/<number>/head` for the PR ref or `pull/<number>/merge` for the merge ref (GitHub only). | [optional]
**review_id** | Option<**String**> | The code review id. | [optional]
**review_url** | Option<**String**> | The code review URL. | [optional]
**revision** | **String** | The code revision the pipeline ran. | 
**tag** | Option<**String**> | The tag used by the pipeline. The commit that this tag points to was used for the pipeline. Note that `branch` and `tag` are mutually exclusive. | [optional]
**commit** | Option<[**crate::models::PipelineVcsCommit**](Pipeline_vcs_commit.md)> |  | [optional]
**origin_repository_url** | **String** | URL for the repository where the trigger originated. For fork-PR pipelines, this is the URL to the fork. For other pipelines the `origin_` and `target_repository_url`s will be the same. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


