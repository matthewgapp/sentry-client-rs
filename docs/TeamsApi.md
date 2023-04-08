# \TeamsApi

All URIs are relative to *https://sentry.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_a_new_project**](TeamsApi.md#create_a_new_project) | **POST** /api/0/teams/{organization_slug}/{team_slug}/projects/ | 
[**create_a_new_team**](TeamsApi.md#create_a_new_team) | **POST** /api/0/organizations/{organization_slug}/teams/ | 
[**delete_a_team**](TeamsApi.md#delete_a_team) | **DELETE** /api/0/teams/{organization_slug}/{team_slug}/ | 
[**list_a_team_quote_s_projects**](TeamsApi.md#list_a_team_quote_s_projects) | **GET** /api/0/teams/{organization_slug}/{team_slug}/projects/ | 
[**list_an_organization_quote_s_teams**](TeamsApi.md#list_an_organization_quote_s_teams) | **GET** /api/0/organizations/{organization_slug}/teams/ | 
[**retrieve_a_team**](TeamsApi.md#retrieve_a_team) | **GET** /api/0/teams/{organization_slug}/{team_slug}/ | 
[**retrieve_event_counts_for_a_team**](TeamsApi.md#retrieve_event_counts_for_a_team) | **GET** /api/0/teams/{organization_slug}/{team_slug}/stats/ | Caution: this endpoint may change in the future without notice.
[**update_a_team**](TeamsApi.md#update_a_team) | **PUT** /api/0/teams/{organization_slug}/{team_slug}/ | 



## create_a_new_project

> crate::models::ListAnOrganizationSTeams200ResponseInnerProjectsInner create_a_new_project(organization_slug, team_slug, create_a_new_project_request)


Create a new project bound to a team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the team belongs to. | [required] |
**team_slug** | **String** | The slug of the team to create a new project for. | [required] |
**create_a_new_project_request** | [**CreateANewProjectRequest**](CreateANewProjectRequest.md) |  | [required] |

### Return type

[**crate::models::ListAnOrganizationSTeams200ResponseInnerProjectsInner**](List_an_Organization_s_Teams_200_response_inner_projects_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_a_new_team

> crate::models::CreateANewTeam201Response create_a_new_team(organization_slug, create_a_new_team_request)


Create a new team bound to an organization. Only the name of the team is needed to create it, the slug can be auto generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the team should be created for. | [required] |
**create_a_new_team_request** | [**CreateANewTeamRequest**](CreateANewTeamRequest.md) |  | [required] |

### Return type

[**crate::models::CreateANewTeam201Response**](Create_a_New_Team_201_response.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_a_team

> delete_a_team(organization_slug, team_slug)


Schedules a team for deletion.  Note: Deletion happens asynchronously and therefore is not immediate. However once deletion has begun the state of a project changes and will be hidden from most public views.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the team belongs to. | [required] |
**team_slug** | **String** | The slug of the team to get. | [required] |

### Return type

 (empty response body)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_a_team_quote_s_projects

> Vec<crate::models::ListATeamSProjects200ResponseInner> list_a_team_quote_s_projects(organization_slug, team_slug, cursor)


Return a list of projects bound to a team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the team belongs to. | [required] |
**team_slug** | **String** | The slug of the team to get. | [required] |
**cursor** | Option<**String**> | A pointer to the last object fetched and its sort order; used to retrieve the next or previous results. |  |

### Return type

[**Vec<crate::models::ListATeamSProjects200ResponseInner>**](List_a_Team_s_Projects_200_response_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_an_organization_quote_s_teams

> Vec<crate::models::ListAnOrganizationSTeams200ResponseInner> list_an_organization_quote_s_teams(organization_slug, cursor)


Returns a list of teams bound to a organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization for which the teams should be listed. | [required] |
**cursor** | Option<**String**> | A pointer to the last object fetched and its sort order; used to retrieve the next or previous results. |  |

### Return type

[**Vec<crate::models::ListAnOrganizationSTeams200ResponseInner>**](List_an_Organization_s_Teams_200_response_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_a_team

> crate::models::RetrieveATeam200Response retrieve_a_team(organization_slug, team_slug)


Return details on an individual team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the team belongs to. | [required] |
**team_slug** | **String** | The slug of the team to get. | [required] |

### Return type

[**crate::models::RetrieveATeam200Response**](Retrieve_a_Team_200_response.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_event_counts_for_a_team

> Vec<Vec<i32>> retrieve_event_counts_for_a_team(organization_slug, team_slug, stat, since, until, resolution)
Caution: this endpoint may change in the future without notice.

Return a set of points representing a normalized timestamp and the number of events seen in the period.  Query ranges are limited to Sentry’s configured time-series resolutions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the team belongs to. | [required] |
**team_slug** | **String** | The slug of the team to get. | [required] |
**stat** | Option<**String**> | The name of the stat to query `(\"received\", \"rejected\")`. |  |
**since** | Option<**String**> | A timestamp to set the start of the query in seconds since UNIX epoch. |  |
**until** | Option<**String**> | A timestamp to set the end of the query in seconds since UNIX epoch. |  |
**resolution** | Option<**String**> | An explicit resolution to search for (one of `10s`, `1h`, and `1d`). |  |

### Return type

[**Vec<Vec<i32>>**](array.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_a_team

> crate::models::CreateANewTeam201Response update_a_team(organization_slug, team_slug, update_a_team_request)


Update various attributes and configurable settings for the given team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the team belongs to. | [required] |
**team_slug** | **String** | The slug of the team to get. | [required] |
**update_a_team_request** | [**UpdateATeamRequest**](UpdateATeamRequest.md) |  | [required] |

### Return type

[**crate::models::CreateANewTeam201Response**](Create_a_New_Team_201_response.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

