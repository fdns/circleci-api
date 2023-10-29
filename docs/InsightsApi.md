# \InsightsApi

All URIs are relative to *https://circleci.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_insights_branches**](InsightsApi.md#get_all_insights_branches) | **GET** /insights/{project-slug}/branches | Get all branches for a project
[**get_flaky_tests**](InsightsApi.md#get_flaky_tests) | **GET** /insights/{project-slug}/flaky-tests | Get flaky tests for a project
[**get_job_timeseries**](InsightsApi.md#get_job_timeseries) | **GET** /insights/time-series/{project-slug}/workflows/{workflow-name}/jobs | Job timeseries data
[**get_org_summary_data**](InsightsApi.md#get_org_summary_data) | **GET** /insights/{org-slug}/summary | Get summary metrics with trends for the entire org, and for each project.
[**get_project_workflow_job_metrics**](InsightsApi.md#get_project_workflow_job_metrics) | **GET** /insights/{project-slug}/workflows/{workflow-name}/jobs | Get summary metrics for a project workflow's jobs.
[**get_project_workflow_metrics**](InsightsApi.md#get_project_workflow_metrics) | **GET** /insights/{project-slug}/workflows | Get summary metrics for a project's workflows
[**get_project_workflow_runs**](InsightsApi.md#get_project_workflow_runs) | **GET** /insights/{project-slug}/workflows/{workflow-name} | Get recent runs of a workflow
[**get_project_workflow_test_metrics**](InsightsApi.md#get_project_workflow_test_metrics) | **GET** /insights/{project-slug}/workflows/{workflow-name}/test-metrics | Get test metrics for a project's workflows
[**get_project_workflows_page_data**](InsightsApi.md#get_project_workflows_page_data) | **GET** /insights/pages/{project-slug}/summary | Get summary metrics and trends for a project across it's workflows and branches
[**get_workflow_summary**](InsightsApi.md#get_workflow_summary) | **GET** /insights/{project-slug}/workflows/{workflow-name}/summary | Get metrics and trends for workflows



## get_all_insights_branches

> serde_json::Value get_all_insights_branches(project_slug, workflow_name)
Get all branches for a project

Get a list of all branches for a specified project. The list will only contain branches currently available within Insights. The maximum number of branches returned by this endpoint is 5,000.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |
**workflow_name** | Option<**String**> | The name of a workflow. If not passed we will scope the API call to the project. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flaky_tests

> crate::models::GetFlakyTests200Response get_flaky_tests(project_slug)
Get flaky tests for a project

Get a list of flaky tests for a given project. Flaky tests are branch agnostic.               A flaky test is a test that passed and failed in the same commit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |

### Return type

[**crate::models::GetFlakyTests200Response**](getFlakyTests_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_timeseries

> crate::models::GetJobTimeseries200Response get_job_timeseries(project_slug, workflow_name, branch, granularity, start_date, end_date)
Job timeseries data

Get timeseries data for all jobs within a workflow. Hourly granularity data is only retained for 48 hours while daily granularity data is retained for 90 days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |
**workflow_name** | **String** | The name of the workflow. | [required] |
**branch** | Option<**String**> | The name of a vcs branch. If not passed we will scope the API call to the default branch. |  |
**granularity** | Option<**String**> | The granularity for which to query timeseries data. |  |
**start_date** | Option<**String**> | Include only executions that started at or after this date. This must be specified if an end-date is provided. |  |
**end_date** | Option<**String**> | Include only executions that started before this date. This date can be at most 90 days after the start-date. |  |

### Return type

[**crate::models::GetJobTimeseries200Response**](getJobTimeseries_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_org_summary_data

> crate::models::GetOrgSummaryData200Response get_org_summary_data(org_slug, reporting_window, project_names)
Get summary metrics with trends for the entire org, and for each project.

Gets aggregated summary metrics with trends for the entire org.                Also gets aggregated metrics and trends for each project belonging to the org.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_slug** | **String** | Org slug in the form `vcs-slug/org-name`. The `/` characters may be URL-escaped. | [required] |
**reporting_window** | Option<**String**> | The time window used to calculate summary metrics. If not provided, defaults to last-90-days |  |
**project_names** | Option<[**serde_json::Value**](.md)> | List of project names. |  |

### Return type

[**crate::models::GetOrgSummaryData200Response**](getOrgSummaryData_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_workflow_job_metrics

> crate::models::GetProjectWorkflowJobMetrics200Response get_project_workflow_job_metrics(project_slug, workflow_name, page_token, all_branches, branch, reporting_window)
Get summary metrics for a project workflow's jobs.

Get summary metrics for a project workflow's jobs. Job runs going back at most 90 days are included in the aggregation window. Metrics are refreshed daily, and thus may not include executions from the last 24 hours. Please note that Insights is not a financial reporting tool and should not be used for precise credit reporting.  Credit reporting from Insights does not use the same source of truth as the billing information that is found in the Plan Overview page in the CircleCI UI, nor does the underlying data have the same data accuracy guarantees as the billing information in the CircleCI UI.  This may lead to discrepancies between credits reported from Insights and the billing information in the Plan Overview page of the CircleCI UI.  For precise credit reporting, always use the Plan Overview page in the CircleCI UI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |
**workflow_name** | **String** | The name of the workflow. | [required] |
**page_token** | Option<**String**> | A token to retrieve the next page of results. |  |
**all_branches** | Option<**bool**> | Whether to retrieve data for all branches combined. Use either this parameter OR the branch name parameter. |  |
**branch** | Option<**String**> | The name of a vcs branch. If not passed we will scope the API call to the default branch. |  |
**reporting_window** | Option<**String**> | The time window used to calculate summary metrics. If not provided, defaults to last-90-days |  |

### Return type

[**crate::models::GetProjectWorkflowJobMetrics200Response**](getProjectWorkflowJobMetrics_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_workflow_metrics

> crate::models::GetProjectWorkflowMetrics200Response get_project_workflow_metrics(project_slug, page_token, all_branches, branch, reporting_window)
Get summary metrics for a project's workflows

Get summary metrics for a project's workflows.  Workflow runs going back at most 90 days are included in the aggregation window. Metrics are refreshed daily, and thus may not include executions from the last 24 hours.  Please note that Insights is not a financial reporting tool and should not be used for precise credit reporting.  Credit reporting from Insights does not use the same source of truth as the billing information that is found in the Plan Overview page in the CircleCI UI, nor does the underlying data have the same data accuracy guarantees as the billing information in the CircleCI UI.  This may lead to discrepancies between credits reported from Insights and the billing information in the Plan Overview page of the CircleCI UI.  For precise credit reporting, always use the Plan Overview page in the CircleCI UI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |
**page_token** | Option<**String**> | A token to retrieve the next page of results. |  |
**all_branches** | Option<**bool**> | Whether to retrieve data for all branches combined. Use either this parameter OR the branch name parameter. |  |
**branch** | Option<**String**> | The name of a vcs branch. If not passed we will scope the API call to the default branch. |  |
**reporting_window** | Option<**String**> | The time window used to calculate summary metrics. If not provided, defaults to last-90-days |  |

### Return type

[**crate::models::GetProjectWorkflowMetrics200Response**](getProjectWorkflowMetrics_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_workflow_runs

> crate::models::GetProjectWorkflowRuns200Response get_project_workflow_runs(project_slug, workflow_name, all_branches, branch, page_token, start_date, end_date)
Get recent runs of a workflow

Get recent runs of a workflow. Runs going back at most 90 days are returned. Please note that Insights is not a financial reporting tool and should not be used for precise credit reporting.  Credit reporting from Insights does not use the same source of truth as the billing information that is found in the Plan Overview page in the CircleCI UI, nor does the underlying data have the same data accuracy guarantees as the billing information in the CircleCI UI.  This may lead to discrepancies between credits reported from Insights and the billing information in the Plan Overview page of the CircleCI UI.  For precise credit reporting, always use the Plan Overview page in the CircleCI UI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |
**workflow_name** | **String** | The name of the workflow. | [required] |
**all_branches** | Option<**bool**> | Whether to retrieve data for all branches combined. Use either this parameter OR the branch name parameter. |  |
**branch** | Option<**String**> | The name of a vcs branch. If not passed we will scope the API call to the default branch. |  |
**page_token** | Option<**String**> | A token to retrieve the next page of results. |  |
**start_date** | Option<**String**> | Include only executions that started at or after this date. This must be specified if an end-date is provided. |  |
**end_date** | Option<**String**> | Include only executions that started before this date. This date can be at most 90 days after the start-date. |  |

### Return type

[**crate::models::GetProjectWorkflowRuns200Response**](getProjectWorkflowRuns_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_workflow_test_metrics

> crate::models::GetProjectWorkflowTestMetrics200Response get_project_workflow_test_metrics(project_slug, workflow_name, branch, all_branches)
Get test metrics for a project's workflows

Get test metrics for a project's workflows. Currently tests metrics are calculated based on 10 most recent workflow runs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |
**workflow_name** | **String** | The name of the workflow. | [required] |
**branch** | Option<**String**> | The name of a vcs branch. If not passed we will scope the API call to the default branch. |  |
**all_branches** | Option<**bool**> | Whether to retrieve data for all branches combined. Use either this parameter OR the branch name parameter. |  |

### Return type

[**crate::models::GetProjectWorkflowTestMetrics200Response**](getProjectWorkflowTestMetrics_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_workflows_page_data

> crate::models::GetProjectWorkflowsPageData200Response get_project_workflows_page_data(project_slug, reporting_window, branches, workflow_names)
Get summary metrics and trends for a project across it's workflows and branches

Get summary metrics and trends for a project at workflow and branch level.               Workflow runs going back at most 90 days are included in the aggregation window.               Trends are only supported upto last 30 days.               Please note that Insights is not a financial reporting tool and should not be used for precise credit reporting.  Credit reporting from Insights does not use the same source of truth as the billing information that is found in the Plan Overview page in the CircleCI UI, nor does the underlying data have the same data accuracy guarantees as the billing information in the CircleCI UI.  This may lead to discrepancies between credits reported from Insights and the billing information in the Plan Overview page of the CircleCI UI.  For precise credit reporting, always use the Plan Overview page in the CircleCI UI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |
**reporting_window** | Option<**String**> | The time window used to calculate summary metrics. If not provided, defaults to last-90-days |  |
**branches** | Option<[**serde_json::Value**](.md)> | The names of VCS branches to include in branch-level workflow metrics. |  |
**workflow_names** | Option<[**serde_json::Value**](.md)> | The names of workflows to include in workflow-level metrics. |  |

### Return type

[**crate::models::GetProjectWorkflowsPageData200Response**](getProjectWorkflowsPageData_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow_summary

> crate::models::GetWorkflowSummary200Response get_workflow_summary(project_slug, workflow_name, all_branches, branch)
Get metrics and trends for workflows

Get the metrics and trends for a particular workflow on a single branch or all branches

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_slug** | **String** | Project slug in the form `vcs-slug/org-name/repo-name`. The `/` characters may be URL-escaped. | [required] |
**workflow_name** | **String** | The name of the workflow. | [required] |
**all_branches** | Option<**bool**> | Whether to retrieve data for all branches combined. Use either this parameter OR the branch name parameter. |  |
**branch** | Option<**String**> | The name of a vcs branch. If not passed we will scope the API call to the default branch. |  |

### Return type

[**crate::models::GetWorkflowSummary200Response**](getWorkflowSummary_200_response.md)

### Authorization

[basic_auth](../README.md#basic_auth), [api_key_query](../README.md#api_key_query), [api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

