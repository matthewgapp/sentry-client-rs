# \ScimApi

All URIs are relative to *https://sentry.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_an_organization_quote_s_paginated_teams**](ScimApi.md#list_an_organization_quote_s_paginated_teams) | **GET** /api/0/organizations/{organization_slug}/scim/v2/Groups | 
[**provision_a_new_team**](ScimApi.md#provision_a_new_team) | **POST** /api/0/organizations/{organization_slug}/scim/v2/Groups | 



## list_an_organization_quote_s_paginated_teams

> crate::models::ListAnOrganizationSPaginatedTeams200Response list_an_organization_quote_s_paginated_teams(organization_slug, start_index, filter, count, excluded_attributes)


Returns a paginated list of teams bound to a organization with a SCIM Groups GET Request. - Note that the members field will only contain up to 10000 members.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization. | [required] |
**start_index** | Option<**i32**> | SCIM 1-offset based index for pagination. |  |
**filter** | Option<**String**> | A SCIM filter expression. The only operator currently supported is `eq`. |  |
**count** | Option<**i32**> | The maximum number of results the query should return, maximum of 100. |  |
**excluded_attributes** | Option<**String**> | Fields that should be left off of return values. Right now the only supported field for this query is `members`. |  |

### Return type

[**crate::models::ListAnOrganizationSPaginatedTeams200Response**](List_an_Organization_s_Paginated_Teams_200_response.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## provision_a_new_team

> crate::models::ListAnOrganizationSPaginatedTeams200ResponseResourcesInner provision_a_new_team(organization_slug, provision_a_new_team_request)


Create a new team bound to an organization via a SCIM Groups POST Request. Note that teams are always created with an empty member set. The endpoint will also do a normalization of uppercase / spaces to lowercase and dashes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization. | [required] |
**provision_a_new_team_request** | [**ProvisionANewTeamRequest**](ProvisionANewTeamRequest.md) |  | [required] |

### Return type

[**crate::models::ListAnOrganizationSPaginatedTeams200ResponseResourcesInner**](List_an_Organization_s_Paginated_Teams_200_response_Resources_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

