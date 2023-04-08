# \OrganizationsApi

All URIs are relative to *https://sentry.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_a_repository_quote_s_commits**](OrganizationsApi.md#list_a_repository_quote_s_commits) | **GET** /api/0/organizations/{organization_slug}/repos/{repo_id}/commits/ | 
[**list_an_organization_quote_s_repositories**](OrganizationsApi.md#list_an_organization_quote_s_repositories) | **GET** /api/0/organizations/{organization_slug}/repos/ | 
[**list_an_organization_quote_s_users**](OrganizationsApi.md#list_an_organization_quote_s_users) | **GET** /api/0/organizations/{organization_slug}/users/ | 
[**list_your_organizations**](OrganizationsApi.md#list_your_organizations) | **GET** /api/0/organizations/ | 
[**resolve_a_short_id**](OrganizationsApi.md#resolve_a_short_id) | **GET** /api/0/organizations/{organization_slug}/shortids/{short_id}/ | 
[**resolve_an_event_id**](OrganizationsApi.md#resolve_an_event_id) | **GET** /api/0/organizations/{organization_slug}/eventids/{event_id}/ | 
[**retrieve_an_organization**](OrganizationsApi.md#retrieve_an_organization) | **GET** /api/0/organizations/{organization_slug}/ | 
[**retrieve_event_counts_for_an_organization**](OrganizationsApi.md#retrieve_event_counts_for_an_organization) | **GET** /api/0/organizations/{organization_slug}/stats/ | 
[**update_an_organization**](OrganizationsApi.md#update_an_organization) | **PUT** /api/0/organizations/{organization_slug}/ | 



## list_a_repository_quote_s_commits

> Vec<crate::models::ListARepositorySCommits200ResponseInner> list_a_repository_quote_s_commits(organization_slug, repo_id)


Return a list of commits for a given repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The organization short name. | [required] |
**repo_id** | **String** | The repository ID. | [required] |

### Return type

[**Vec<crate::models::ListARepositorySCommits200ResponseInner>**](List_a_Repository_s_Commits_200_response_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_an_organization_quote_s_repositories

> Vec<crate::models::ListAnOrganizationSRepositories200ResponseInner> list_an_organization_quote_s_repositories(organization_slug)


Return a list of version control repositories for a given organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The organization short name. | [required] |

### Return type

[**Vec<crate::models::ListAnOrganizationSRepositories200ResponseInner>**](List_an_Organization_s_Repositories_200_response_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_an_organization_quote_s_users

> Vec<crate::models::ListAnOrganizationSUsers200ResponseInner> list_an_organization_quote_s_users(organization_slug, project)


Return a list of users that belong to a given organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the event ID should be looked up in. | [required] |
**project** | Option<**String**> | Restrict results to users who have access to a given project ID |  |

### Return type

[**Vec<crate::models::ListAnOrganizationSUsers200ResponseInner>**](List_an_Organization_s_Users_200_response_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_your_organizations

> Vec<crate::models::RetrieveATeam200ResponseOrganization> list_your_organizations(owner, cursor)


Return a list of organizations available to the authenticated session.  This is particularly useful for requests with an user bound context.  For API key based requests this will only return the organization that belongs to the key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | Option<**bool**> | Restrict results to organizations in which you are an organization owner. |  |
**cursor** | Option<**String**> | A pointer to the last object fetched and its sort order; used to retrieve the next or previous results. |  |

### Return type

[**Vec<crate::models::RetrieveATeam200ResponseOrganization>**](Retrieve_a_Team_200_response_organization.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resolve_a_short_id

> crate::models::ResolveAShortId200Response resolve_a_short_id(organization_slug, short_id)


This resolves a short ID to the project slug and internal issue ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the short ID should be looked up in. | [required] |
**short_id** | **String** | The short ID to look up. | [required] |

### Return type

[**crate::models::ResolveAShortId200Response**](Resolve_a_Short_ID_200_response.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resolve_an_event_id

> crate::models::ResolveAnEventId200Response resolve_an_event_id(organization_slug, event_id)


This resolves an event ID to the project slug and internal issue ID and internal event ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the event ID should be looked up in. | [required] |
**event_id** | **String** | The event ID to look up. | [required] |

### Return type

[**crate::models::ResolveAnEventId200Response**](Resolve_an_Event_ID_200_response.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_an_organization

> crate::models::RetrieveAnOrganization200Response retrieve_an_organization(organization_slug)


Return details on an individual organization including various details such as membership access, features, and teams.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization to look up. | [required] |

### Return type

[**crate::models::RetrieveAnOrganization200Response**](Retrieve_an_Organization_200_response.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_event_counts_for_an_organization

> Vec<Vec<i32>> retrieve_event_counts_for_an_organization(organization_slug, stat, since, until, resolution)


This endpoint is deprecated in favor of [Organization Stats V2](/api/organizations/retrieve-event-counts-for-an-organization-v2/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the event ID should be looked up in. | [required] |
**stat** | Option<**String**> | The name of the stat to query `(\"received\", \"rejected\", \"blacklisted\")`. |  |
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


## update_an_organization

> crate::models::RetrieveAnOrganization200Response update_an_organization(organization_slug, update_an_organization_request)


Update various attributes and configurable settings for the given organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization to update. | [required] |
**update_an_organization_request** | Option<[**UpdateAnOrganizationRequest**](UpdateAnOrganizationRequest.md)> |  |  |

### Return type

[**crate::models::RetrieveAnOrganization200Response**](Retrieve_an_Organization_200_response.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

