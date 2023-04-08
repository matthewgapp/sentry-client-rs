# \IntegrationApi

All URIs are relative to *https://sentry.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_an_external_issue**](IntegrationApi.md#create_an_external_issue) | **POST** /api/0/sentry-app-installations/{uuid}/external-issues/ | 
[**delete_an_external_issue**](IntegrationApi.md#delete_an_external_issue) | **DELETE** /api/0/sentry-app-installations/{uuid}/external-issues/{external_issue_id}/ | 
[**list_an_organization_quote_s_integration_platform_installations**](IntegrationApi.md#list_an_organization_quote_s_integration_platform_installations) | **GET** /api/0/organizations/{organization_slug}/sentry-app-installations/ | 



## create_an_external_issue

> crate::models::CreateAnExternalIssue200Response create_an_external_issue(uuid, create_an_external_issue_request)


Create an external issue from an integration platform integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | The uuid of the integration platform integration. | [required] |
**create_an_external_issue_request** | [**CreateAnExternalIssueRequest**](CreateAnExternalIssueRequest.md) |  | [required] |

### Return type

[**crate::models::CreateAnExternalIssue200Response**](Create_an_External_Issue_200_response.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_an_external_issue

> delete_an_external_issue(uuid, external_issue_id)


Delete an external issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | The uuid of the integration platform integration. | [required] |
**external_issue_id** | **String** | The id of the external issue. | [required] |

### Return type

 (empty response body)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_an_organization_quote_s_integration_platform_installations

> Vec<crate::models::ListAnOrganizationSIntegrationPlatformInstallations200ResponseInner> list_an_organization_quote_s_integration_platform_installations(organization_slug)


Return a list of integration platform installations for a given organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The organization short name. | [required] |

### Return type

[**Vec<crate::models::ListAnOrganizationSIntegrationPlatformInstallations200ResponseInner>**](List_an_Organization_s_Integration_Platform_Installations_200_response_inner.md)

### Authorization

[auth_token](../README.md#auth_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

