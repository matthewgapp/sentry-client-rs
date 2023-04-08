# \EventsApi

All URIs are relative to *https://sentry.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_mutate_a_list_of_issues**](EventsApi.md#bulk_mutate_a_list_of_issues) | **PUT** /api/0/projects/{organization_slug}/{project_slug}/issues/ | 
[**bulk_remove_a_list_of_issues**](EventsApi.md#bulk_remove_a_list_of_issues) | **DELETE** /api/0/projects/{organization_slug}/{project_slug}/issues/ | 
[**list_a_project_quote_s_events**](EventsApi.md#list_a_project_quote_s_events) | **GET** /api/0/projects/{organization_slug}/{project_slug}/events/ | 
[**list_a_project_quote_s_issues**](EventsApi.md#list_a_project_quote_s_issues) | **GET** /api/0/projects/{organization_slug}/{project_slug}/issues/ | 
[**list_a_tag_quote_s_values_related_to_an_issue**](EventsApi.md#list_a_tag_quote_s_values_related_to_an_issue) | **GET** /api/0/issues/{issue_id}/tags/{key}/values/ | 
[**list_an_issue_quote_s_events**](EventsApi.md#list_an_issue_quote_s_events) | **GET** /api/0/issues/{issue_id}/events/ | 
[**list_an_issue_quote_s_hashes**](EventsApi.md#list_an_issue_quote_s_hashes) | **GET** /api/0/issues/{issue_id}/hashes/ | 
[**remove_an_issue**](EventsApi.md#remove_an_issue) | **DELETE** /api/0/issues/{issue_id}/ | 
[**retrieve_an_event_for_a_project**](EventsApi.md#retrieve_an_event_for_a_project) | **GET** /api/0/projects/{organization_slug}/{project_slug}/events/{event_id}/ | 
[**retrieve_an_issue**](EventsApi.md#retrieve_an_issue) | **GET** /api/0/issues/{issue_id}/ | 
[**retrieve_tag_details**](EventsApi.md#retrieve_tag_details) | **GET** /api/0/issues/{issue_id}/tags/{key}/ | 
[**retrieve_the_latest_event_for_an_issue**](EventsApi.md#retrieve_the_latest_event_for_an_issue) | **GET** /api/0/issues/{issue_id}/events/latest/ | 
[**retrieve_the_oldest_event_for_an_issue**](EventsApi.md#retrieve_the_oldest_event_for_an_issue) | **GET** /api/0/issues/{issue_id}/events/oldest/ | 
[**update_an_issue**](EventsApi.md#update_an_issue) | **PUT** /api/0/issues/{issue_id}/ | 



## bulk_mutate_a_list_of_issues

> crate::models::BulkMutateAListOfIssues200Response bulk_mutate_a_list_of_issues(organization_slug, project_slug, bulk_mutate_a_list_of_issues_request, id, status)


Bulk mutate various attributes on issues.  The list of issues to modify is given through the `id` query parameter.  It is repeated for each issue that should be modified.  - For non-status updates, the `id` query parameter is required. - For status updates, the `id` query parameter may be omitted for a batch \"update all\" query. - An optional `status` query parameter may be used to restrict mutations to only events with the given status.  The following attributes can be modified and are supplied as JSON object in the body:  If any ids are out of scope this operation will succeed without any data mutation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the issues belong to. | [required] |
**project_slug** | **String** | The slug of the project the issues belong to. | [required] |
**bulk_mutate_a_list_of_issues_request** | [**BulkMutateAListOfIssuesRequest**](BulkMutateAListOfIssuesRequest.md) |  | [required] |
**id** | Option<**i32**> | A list of IDs of the issues to be mutated. This parameter shall be repeated for each issue. It is optional only if a status is mutated in which case an implicit update all is assumed. |  |
**status** | Option<**String**> | Optionally limits the query to issues of the specified status. Valid values are `\"resolved\"`, `\"unresolved\"`, and `\"ignored\"`. |  |

### Return type

[**crate::models::BulkMutateAListOfIssues200Response**](Bulk_Mutate_a_List_of_Issues_200_response.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_remove_a_list_of_issues

> bulk_remove_a_list_of_issues(organization_slug, project_slug, id)


Permanently remove the given issues. The list of issues to modify is given through the `id` query parameter.  It is repeated for each issue that should be removed.  Only queries by 'id' are accepted.  If any ids are out of scope this operation will succeed without any data mutation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the issues belong to. | [required] |
**project_slug** | **String** | The slug of the project the issues belong to. | [required] |
**id** | Option<**i32**> | A list of IDs of the issues to be removed. This parameter shall be repeated for each issue. |  |

### Return type

 (empty response body)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_a_project_quote_s_events

> Vec<crate::models::ListAProjectSEvents200ResponseInner> list_a_project_quote_s_events(organization_slug, project_slug, full, cursor)


Return a list of events bound to a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the groups belong to. | [required] |
**project_slug** | **String** | The slug of the project the groups belong to. | [required] |
**full** | Option<**bool**> | If this is set to true then the event payload will include the full event body, including the stacktrace.  Set to true to enable. |  |
**cursor** | Option<**String**> | A pointer to the last object fetched and its sort order; used to retrieve the next or previous results. |  |

### Return type

[**Vec<crate::models::ListAProjectSEvents200ResponseInner>**](List_a_Project_s_Events_200_response_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_a_project_quote_s_issues

> Vec<crate::models::ListAProjectSIssues200ResponseInner> list_a_project_quote_s_issues(organization_slug, project_slug, stats_period, short_id_lookup, query, cursor)


Return a list of issues (groups) bound to a project.  All parameters are supplied as query string parameters.    A default query of ``is:unresolved`` is applied. To return results with other statuses send an new query value (i.e. ``?query=`` for all results).  The ``statsPeriod`` parameter can be used to select the timeline stats which should be present. Possible values are: ``\"\"`` (disable),``\"24h\"``, ``\"14d\"``

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the issues belong to. | [required] |
**project_slug** | **String** | The slug of the project the issues belong to. | [required] |
**stats_period** | Option<**String**> | An optional stat period (can be one of `\"24h\"`, `\"14d\"`, and `\"\"`). |  |
**short_id_lookup** | Option<**bool**> | If this is set to true then short IDs are looked up by this function as well. This can cause the return value of the function to return an event issue of a different project which is why this is an opt-in. Set to 1 to enable. |  |
**query** | Option<**String**> | An optional Sentry structured search query. If not provided an implied `\"is:unresolved\"` is assumed. |  |
**cursor** | Option<**String**> | A pointer to the last object fetched and its sort order; used to retrieve the next or previous results. |  |

### Return type

[**Vec<crate::models::ListAProjectSIssues200ResponseInner>**](List_a_Project_s_Issues_200_response_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_a_tag_quote_s_values_related_to_an_issue

> Vec<crate::models::ListATagSValuesRelatedToAnIssue200ResponseInner> list_a_tag_quote_s_values_related_to_an_issue(issue_id, key)


Returns details for given tag key related to an issue.   When [paginated](/api/pagination) can return at most 1000 values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The ID of the issue to retrieve. | [required] |
**key** | **String** | The tag key to look the values up for. | [required] |

### Return type

[**Vec<crate::models::ListATagSValuesRelatedToAnIssue200ResponseInner>**](List_a_Tag_s_Values_Related_to_an_Issue_200_response_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_an_issue_quote_s_events

> Vec<crate::models::ListAProjectSEvents200ResponseInner> list_an_issue_quote_s_events(issue_id, full)


This endpoint lists an issue's events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The ID of the issue to retrieve. | [required] |
**full** | Option<**bool**> | If this is set to true then the event payload will include the full event body, including the stacktrace.  Set to true to enable. |  |

### Return type

[**Vec<crate::models::ListAProjectSEvents200ResponseInner>**](List_a_Project_s_Events_200_response_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_an_issue_quote_s_hashes

> Vec<crate::models::ListAnIssueSHashes200ResponseInner> list_an_issue_quote_s_hashes(issue_id, cursor)


This endpoint lists an issue's hashes, which are the generated checksums used to aggregate individual events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The ID of the issue to retrieve. | [required] |
**cursor** | Option<**String**> | A pointer to the last object fetched and its sort order; used to retrieve the next or previous results. |  |

### Return type

[**Vec<crate::models::ListAnIssueSHashes200ResponseInner>**](List_an_Issue_s_Hashes_200_response_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_an_issue

> remove_an_issue(issue_id)


Removes an individual issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The ID of the issue to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_an_event_for_a_project

> crate::models::RetrieveAnEventForAProject200Response retrieve_an_event_for_a_project(organization_slug, project_slug, event_id)


Return details on an individual event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The slug of the organization the event belongs to. | [required] |
**project_slug** | **String** | The slug of the project the event belongs to. | [required] |
**event_id** | **String** | The ID of the event to retrieve. It is the hexadecimal ID as reported by the client. | [required] |

### Return type

[**crate::models::RetrieveAnEventForAProject200Response**](Retrieve_an_Event_for_a_Project_200_response.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_an_issue

> crate::models::RetrieveAnIssue200Response retrieve_an_issue(issue_id)


Return details on an individual issue. This returns the basic stats for the issue (title, last seen, first seen), some overall numbers (number of comments, user reports) as well as the summarized event data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The ID of the issue to retrieve. | [required] |

### Return type

[**crate::models::RetrieveAnIssue200Response**](Retrieve_an_Issue_200_response.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_tag_details

> crate::models::RetrieveTagDetails200Response retrieve_tag_details(issue_id, key)


Returns details for given tag key related to an issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The ID of the issue to retrieve. | [required] |
**key** | **String** | The tag key to look the values up for. | [required] |

### Return type

[**crate::models::RetrieveTagDetails200Response**](Retrieve_Tag_Details_200_response.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_the_latest_event_for_an_issue

> crate::models::RetrieveAnEventForAProject200Response retrieve_the_latest_event_for_an_issue(issue_id)


Retrieves the details of the latest event for an issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The ID of the issue. | [required] |

### Return type

[**crate::models::RetrieveAnEventForAProject200Response**](Retrieve_an_Event_for_a_Project_200_response.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_the_oldest_event_for_an_issue

> crate::models::RetrieveAnEventForAProject200Response retrieve_the_oldest_event_for_an_issue(issue_id)


Retrieves the details of the oldest event for an issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The ID of the issue. | [required] |

### Return type

[**crate::models::RetrieveAnEventForAProject200Response**](Retrieve_an_Event_for_a_Project_200_response.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_an_issue

> crate::models::UpdateAnIssue200Response update_an_issue(issue_id, update_an_issue_request)


Updates an individual issue's attributes.  Only the attributes submitted are modified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id** | **String** | The ID of the group to retrieve. | [required] |
**update_an_issue_request** | [**UpdateAnIssueRequest**](UpdateAnIssueRequest.md) |  | [required] |

### Return type

[**crate::models::UpdateAnIssue200Response**](Update_an_Issue_200_response.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

