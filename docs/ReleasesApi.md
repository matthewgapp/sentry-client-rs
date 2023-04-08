# \ReleasesApi

All URIs are relative to *https://sentry.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_a_new_deploy_for_an_organization**](ReleasesApi.md#create_a_new_deploy_for_an_organization) | **POST** /api/0/organizations/{organization_slug}/releases/{version}/deploys/ | 
[**create_a_new_release_for_an_organization**](ReleasesApi.md#create_a_new_release_for_an_organization) | **POST** /api/0/organizations/{organization_slug}/releases/ | 
[**delete_a_project_release_quote_s_file**](ReleasesApi.md#delete_a_project_release_quote_s_file) | **DELETE** /api/0/projects/{organization_slug}/{project_slug}/releases/{version}/files/{file_id}/ | 
[**delete_an_organization_quote_s_release**](ReleasesApi.md#delete_an_organization_quote_s_release) | **DELETE** /api/0/organizations/{organization_slug}/releases/{version}/ | 
[**delete_an_organization_release_quote_s_file**](ReleasesApi.md#delete_an_organization_release_quote_s_file) | **DELETE** /api/0/organizations/{organization_slug}/releases/{version}/files/{file_id}/ | 
[**list_a_project_quote_s_release_files**](ReleasesApi.md#list_a_project_quote_s_release_files) | **GET** /api/0/projects/{organization_slug}/{project_slug}/releases/{version}/files/ | 
[**list_a_project_release_quote_s_commits**](ReleasesApi.md#list_a_project_release_quote_s_commits) | **GET** /api/0/projects/{organization_slug}/{project_slug}/releases/{version}/commits/ | 
[**list_a_release_quote_s_deploys**](ReleasesApi.md#list_a_release_quote_s_deploys) | **GET** /api/0/organizations/{organization_slug}/releases/{version}/deploys/ | 
[**list_an_organization_quote_s_release_files**](ReleasesApi.md#list_an_organization_quote_s_release_files) | **GET** /api/0/organizations/{organization_slug}/releases/{version}/files/ | 
[**list_an_organization_quote_s_releases**](ReleasesApi.md#list_an_organization_quote_s_releases) | **GET** /api/0/organizations/{organization_slug}/releases/ | 
[**list_an_organization_release_quote_s_commits**](ReleasesApi.md#list_an_organization_release_quote_s_commits) | **GET** /api/0/organizations/{organization_slug}/releases/{version}/commits/ | 
[**list_issues_to_be_resolved_in_a_particular_release**](ReleasesApi.md#list_issues_to_be_resolved_in_a_particular_release) | **GET** /api/0/projects/{organization_slug}/{project_slug}/releases/{version}/resolved/ | 
[**retrieve_a_project_release_quote_s_file**](ReleasesApi.md#retrieve_a_project_release_quote_s_file) | **GET** /api/0/projects/{organization_slug}/{project_slug}/releases/{version}/files/{file_id}/ | 
[**retrieve_an_organization_quote_s_releases**](ReleasesApi.md#retrieve_an_organization_quote_s_releases) | **GET** /api/0/organizations/{organization_slug}/releases/{version}/ | 
[**retrieve_an_organization_release_quote_s_file**](ReleasesApi.md#retrieve_an_organization_release_quote_s_file) | **GET** /api/0/organizations/{organization_slug}/releases/{version}/files/{file_id}/ | 
[**retrieve_files_changed_in_a_release_quote_s_commits**](ReleasesApi.md#retrieve_files_changed_in_a_release_quote_s_commits) | **GET** /api/0/organizations/{organization_slug}/releases/{version}/commitfiles/ | 
[**retrieve_release_health_session_statistics**](ReleasesApi.md#retrieve_release_health_session_statistics) | **GET** /api/0/organizations/{organization_slug}/sessions/ | 
[**update_a_project_release_file**](ReleasesApi.md#update_a_project_release_file) | **PUT** /api/0/projects/{organization_slug}/{project_slug}/releases/{version}/files/{file_id}/ | 
[**update_an_organization_quote_s_release**](ReleasesApi.md#update_an_organization_quote_s_release) | **PUT** /api/0/organizations/{organization_slug}/releases/{version}/ | 
[**update_an_organization_release_file**](ReleasesApi.md#update_an_organization_release_file) | **PUT** /api/0/organizations/{organization_slug}/releases/{version}/files/{file_id}/ | 
[**upload_a_new_organization_release_file**](ReleasesApi.md#upload_a_new_organization_release_file) | **POST** /api/0/organizations/{organization_slug}/releases/{version}/files/ | 
[**upload_a_new_project_release_file**](ReleasesApi.md#upload_a_new_project_release_file) | **POST** /api/0/projects/{organization_slug}/{project_slug}/releases/{version}/files/ | 



## create_a_new_deploy_for_an_organization

> crate::models::RetrieveAnEventForAProject200ResponseReleaseOneOfLastDeployOneOf create_a_new_deploy_for_an_organization(organization_slug, version, create_a_new_deploy_for_an_organization_request)


Create a deploy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization. | [required] |
**version** | **String** | The version identifier of the release. | [required] |
**create_a_new_deploy_for_an_organization_request** | Option<[**CreateANewDeployForAnOrganizationRequest**](CreateANewDeployForAnOrganizationRequest.md)> |  |  |

### Return type

[**crate::models::RetrieveAnEventForAProject200ResponseReleaseOneOfLastDeployOneOf**](Retrieve_an_Event_for_a_Project_200_response_release_oneOf_lastDeploy_oneOf.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_a_new_release_for_an_organization

> crate::models::RetrieveAnEventForAProject200ResponseReleaseOneOf create_a_new_release_for_an_organization(organization_slug, create_a_new_release_for_an_organization_request)


Create a new release for the given organization.  Releases are used by Sentry to improve its error reporting abilities by correlating first seen events with the release that might have introduced the problem. Releases are also necessary for source maps and other debug features that require manual upload for functioning well.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization. | [required] |
**create_a_new_release_for_an_organization_request** | Option<[**CreateANewReleaseForAnOrganizationRequest**](CreateANewReleaseForAnOrganizationRequest.md)> |  |  |

### Return type

[**crate::models::RetrieveAnEventForAProject200ResponseReleaseOneOf**](Retrieve_an_Event_for_a_Project_200_response_release_oneOf.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_a_project_release_quote_s_file

> delete_a_project_release_quote_s_file(organization_slug, project_slug, version, file_id)


Delete a file for a given release.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the release belongs to. | [required] |
**project_slug** | **String** | The slug of the project. | [required] |
**version** | **String** | The version identifier of the release. | [required] |
**file_id** | **String** | The ID of the file to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_an_organization_quote_s_release

> delete_an_organization_quote_s_release(organization_slug, version)


Delete a release for a given organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the release belongs to. | [required] |
**version** | **String** | The version identifier of the release. | [required] |

### Return type

 (empty response body)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_an_organization_release_quote_s_file

> delete_an_organization_release_quote_s_file(organization_slug, version, file_id)


Delete a file for a given release.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the release belongs to. | [required] |
**version** | **String** | The version identifier of the release. | [required] |
**file_id** | **String** | The ID of the file to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_a_project_quote_s_release_files

> Vec<crate::models::ListAnOrganizationSReleaseFiles200ResponseInner> list_a_project_quote_s_release_files(organization_slug, project_slug, version)


Return a list of files for a given release.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization. | [required] |
**project_slug** | **String** | The slug of the project. | [required] |
**version** | **String** | The version identifier of the release. | [required] |

### Return type

[**Vec<crate::models::ListAnOrganizationSReleaseFiles200ResponseInner>**](List_an_Organization_s_Release_Files_200_response_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_a_project_release_quote_s_commits

> Vec<crate::models::ListARepositorySCommits200ResponseInner> list_a_project_release_quote_s_commits(organization_slug, project_slug, version)


List a project release's commits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the release belongs to. | [required] |
**project_slug** | **String** | The slug of the project the release belongs to. | [required] |
**version** | **String** | The version identifier of the release. | [required] |

### Return type

[**Vec<crate::models::ListARepositorySCommits200ResponseInner>**](List_a_Repository_s_Commits_200_response_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_a_release_quote_s_deploys

> Vec<crate::models::RetrieveAnEventForAProject200ResponseReleaseOneOfLastDeployOneOf> list_a_release_quote_s_deploys(organization_slug, version)


Return a list of deploys for a given release.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization. | [required] |
**version** | **String** | The version identifier of the release. | [required] |

### Return type

[**Vec<crate::models::RetrieveAnEventForAProject200ResponseReleaseOneOfLastDeployOneOf>**](Retrieve_an_Event_for_a_Project_200_response_release_oneOf_lastDeploy_oneOf.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_an_organization_quote_s_release_files

> Vec<crate::models::ListAnOrganizationSReleaseFiles200ResponseInner> list_an_organization_quote_s_release_files(organization_slug, version)


Return a list of files for a given release.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization. | [required] |
**version** | **String** | The version identifier of the release. | [required] |

### Return type

[**Vec<crate::models::ListAnOrganizationSReleaseFiles200ResponseInner>**](List_an_Organization_s_Release_Files_200_response_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_an_organization_quote_s_releases

> Vec<crate::models::RetrieveAnEventForAProject200ResponseReleaseOneOf> list_an_organization_quote_s_releases(organization_slug, query)


Return a list of releases for a given organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization. | [required] |
**query** | Option<**String**> | This parameter can be used to create a \"starts with\" filter for the version. |  |

### Return type

[**Vec<crate::models::RetrieveAnEventForAProject200ResponseReleaseOneOf>**](Retrieve_an_Event_for_a_Project_200_response_release_oneOf.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_an_organization_release_quote_s_commits

> Vec<crate::models::ListARepositorySCommits200ResponseInner> list_an_organization_release_quote_s_commits(organization_slug, version)


List an organization release's commits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the release belongs to. | [required] |
**version** | **String** | The version identifier of the release. | [required] |

### Return type

[**Vec<crate::models::ListARepositorySCommits200ResponseInner>**](List_a_Repository_s_Commits_200_response_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_issues_to_be_resolved_in_a_particular_release

> list_issues_to_be_resolved_in_a_particular_release(organization_slug, project_slug, version)


List issues to be resolved in a particular release.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization. | [required] |
**project_slug** | **String** | The slug of the project. | [required] |
**version** | **String** | The version identifier of the release. | [required] |

### Return type

 (empty response body)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_a_project_release_quote_s_file

> crate::models::ListAnOrganizationSReleaseFiles200ResponseInner retrieve_a_project_release_quote_s_file(organization_slug, project_slug, version, file_id)


Retrieve a file for a given release.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization. | [required] |
**project_slug** | **String** | The slug of the project. | [required] |
**version** | **String** | The version identifier of the release. | [required] |
**file_id** | **String** | The ID of the file to retrieve. | [required] |

### Return type

[**crate::models::ListAnOrganizationSReleaseFiles200ResponseInner**](List_an_Organization_s_Release_Files_200_response_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_an_organization_quote_s_releases

> crate::models::RetrieveAnEventForAProject200ResponseReleaseOneOf retrieve_an_organization_quote_s_releases(organization_slug, version)


Return a release for a given organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the release belongs to. | [required] |
**version** | **String** | The version identifier of the release. | [required] |

### Return type

[**crate::models::RetrieveAnEventForAProject200ResponseReleaseOneOf**](Retrieve_an_Event_for_a_Project_200_response_release_oneOf.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_an_organization_release_quote_s_file

> crate::models::ListAnOrganizationSReleaseFiles200ResponseInner retrieve_an_organization_release_quote_s_file(organization_slug, version, file_id)


Retrieve a file for a given release.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization. | [required] |
**version** | **String** | The version identifier of the release. | [required] |
**file_id** | **String** | The ID of the file to retrieve. | [required] |

### Return type

[**crate::models::ListAnOrganizationSReleaseFiles200ResponseInner**](List_an_Organization_s_Release_Files_200_response_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_files_changed_in_a_release_quote_s_commits

> retrieve_files_changed_in_a_release_quote_s_commits(organization_slug, version)


Retrieve files changed in a release's commits

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the release belongs to. | [required] |
**version** | **String** | The version identifier of the release. | [required] |

### Return type

 (empty response body)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_release_health_session_statistics

> crate::models::RetrieveReleaseHealthSessionStatistics200Response retrieve_release_health_session_statistics(organization_slug, project, field, environment, group_by, order_by, query, stats_period, interval, stats_period_start, stats_period_end, start, end)


Returns a time series of release health session statistics for projects bound to an organization.  The interval and date range are subject to certain restrictions and rounding rules.  The date range is rounded to align with the interval, and is rounded to at least one hour. The interval can at most be one day and at least one hour currently. It has to cleanly divide one day, for rounding reasons.  Apart from the query parameters listed below, this endpoint also supports the usual [pagination parameters](https://docs.sentry.io/api/pagination/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization. | [required] |
**project** | [**Vec<i32>**](i32.md) | The ID of the projects to filter by.  Use `-1` to include all accessible projects. | [required] |
**field** | [**Vec<String>**](String.md) | The list of fields to query.  The available fields are   - `sum(session)`   - `count_unique(user)`   - `avg`, `p50`, `p75`, `p90`, `p95`, `p99`, `max` applied to `session.duration`. For example, `p99(session.duration)`. Session duration is [no longer being recorded](https://github.com/getsentry/sentry/discussions/42716) as of on Jan 12, 2023. Returned data may be incomplete.   - `crash_rate`, `crash_free_rate` applied to `user` or `session`. For example, `crash_free_rate(user)` | [required] |
**environment** | Option<[**Vec<String>**](String.md)> | The name of environments to filter by. |  |
**group_by** | Option<[**Vec<String>**](String.md)> | The list of properties to group by.  The available groupBy conditions are `project`, `release`, `environment` and `session.status`.  Grouping by `session.status` does not work when `crash_rate` or `crash_free_rate` are queried. |  |
**order_by** | Option<**String**> | An optional field to order by, which must be one of the fields provided in `field`. Use `-` for descending order, for example `-sum(session)`.   This alters the order of the `groups` returned, so it only makes sense in combination with `groupBy`.   Ordering by more than one field is currently not supported. |  |
**query** | Option<**String**> | A free-form query that is applied as a filter.  An example query could be `release:\"1.1.0\" or release:\"1.2.0\"`. |  |
**stats_period** | Option<**String**> | This defines the range of the time series, relative to now.  The range is given in a `\"<number><unit>\"` format.  For example `1d` for a one day range. Possible units are `m` for minutes, `h` for hours, `d` for days and `w` for weeks.  It defaults to `90d`. |  |
**interval** | Option<**String**> | This is the resolution of the time series, given in the same format as `statsPeriod`.  The default resolution is `1h` and the minimum resolution is currently restricted to `1h` as well.  Intervals larger than `1d` are not supported, and the interval has to cleanly divide one day. |  |
**stats_period_start** | Option<**String**> | This defines the start of the time series range, in the same format as the `interval`, relative to now. |  |
**stats_period_end** | Option<**String**> | This defines the end of the time series range, in the same format as the `interval`, relative to now. |  |
**start** | Option<**String**> | This defines the start of the time series range as an explicit datetime. |  |
**end** | Option<**String**> | This defines the inclusive end of the time series range as an explicit datetime. |  |

### Return type

[**crate::models::RetrieveReleaseHealthSessionStatistics200Response**](Retrieve_Release_Health_Session_Statistics_200_response.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_a_project_release_file

> crate::models::ListAnOrganizationSReleaseFiles200ResponseInner update_a_project_release_file(organization_slug, project_slug, version, file_id, update_an_organization_release_file_request)


Update a project release file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization. | [required] |
**project_slug** | **String** | The slug of the project. | [required] |
**version** | **String** | The version identifier of the release. | [required] |
**file_id** | **String** | The ID of the file to retrieve. | [required] |
**update_an_organization_release_file_request** | Option<[**UpdateAnOrganizationReleaseFileRequest**](UpdateAnOrganizationReleaseFileRequest.md)> |  |  |

### Return type

[**crate::models::ListAnOrganizationSReleaseFiles200ResponseInner**](List_an_Organization_s_Release_Files_200_response_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_an_organization_quote_s_release

> crate::models::RetrieveAnEventForAProject200ResponseReleaseOneOf update_an_organization_quote_s_release(organization_slug, version, update_an_organization_s_release_request)


Update a release for a given organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the release belongs to. | [required] |
**version** | **String** | The version identifier of the release. | [required] |
**update_an_organization_s_release_request** | Option<[**UpdateAnOrganizationSReleaseRequest**](UpdateAnOrganizationSReleaseRequest.md)> |  |  |

### Return type

[**crate::models::RetrieveAnEventForAProject200ResponseReleaseOneOf**](Retrieve_an_Event_for_a_Project_200_response_release_oneOf.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_an_organization_release_file

> crate::models::ListAnOrganizationSReleaseFiles200ResponseInner update_an_organization_release_file(organization_slug, version, file_id, update_an_organization_release_file_request)


Update an organization release file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization. | [required] |
**version** | **String** | The version identifier of the release. | [required] |
**file_id** | **String** | The ID of the file to retrieve. | [required] |
**update_an_organization_release_file_request** | Option<[**UpdateAnOrganizationReleaseFileRequest**](UpdateAnOrganizationReleaseFileRequest.md)> |  |  |

### Return type

[**crate::models::ListAnOrganizationSReleaseFiles200ResponseInner**](List_an_Organization_s_Release_Files_200_response_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_a_new_organization_release_file

> upload_a_new_organization_release_file(organization_slug, version, file, name, dist, header)


Upload a new organization release file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization. | [required] |
**version** | **String** | The version identifier of the release. | [required] |
**file** | **std::path::PathBuf** | The multipart encoded file. | [required] |
**name** | Option<**String**> | The name (full path) of the file. |  |
**dist** | Option<**String**> | The name of the dist. |  |
**header** | Option<**String**> | This parameter can be supplied multiple times to attach headers to the file. Each header is a string in the format `key:value`. For instance it can be used to define a content type. |  |

### Return type

 (empty response body)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_a_new_project_release_file

> crate::models::ListAnOrganizationSReleaseFiles200ResponseInner upload_a_new_project_release_file(organization_slug, project_slug, version, file, name, dist, header)


Upload a new project release file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization. | [required] |
**project_slug** | **String** | The slug of the project. | [required] |
**version** | **String** | The version identifier of the release. | [required] |
**file** | **std::path::PathBuf** | The multipart encoded file. | [required] |
**name** | Option<**String**> | The name (full path) of the file. |  |
**dist** | Option<**String**> | The name of the dist. |  |
**header** | Option<**String**> | This parameter can be supplied multiple times to attach headers to the file. Each header is a string in the format `key:value`. For instance it can be used to define a content type. |  |

### Return type

[**crate::models::ListAnOrganizationSReleaseFiles200ResponseInner**](List_an_Organization_s_Release_Files_200_response_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

