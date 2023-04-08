# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_pull_requests_by_id**](DefaultApi.md#get_pull_requests_by_id) | **GET** /2.0/repositories/{username}/{slug}/pullrequests/{pid} | 
[**get_pull_requests_by_repository**](DefaultApi.md#get_pull_requests_by_repository) | **GET** /2.0/repositories/{username}/{slug}/pullrequests | 
[**get_repositories_by_owner**](DefaultApi.md#get_repositories_by_owner) | **GET** /2.0/repositories/{username} | 
[**get_repository**](DefaultApi.md#get_repository) | **GET** /2.0/repositories/{username}/{slug} | 
[**get_user_by_name**](DefaultApi.md#get_user_by_name) | **GET** /2.0/users/{username} | 
[**merge_pull_request**](DefaultApi.md#merge_pull_request) | **POST** /2.0/repositories/{username}/{slug}/pullrequests/{pid}/merge | 



## get_pull_requests_by_id

> crate::models::Pullrequest get_pull_requests_by_id(username, slug, pid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**slug** | **String** |  | [required] |
**pid** | **String** |  | [required] |

### Return type

[**crate::models::Pullrequest**](pullrequest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pull_requests_by_repository

> Vec<crate::models::Pullrequest> get_pull_requests_by_repository(username, slug, state)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**slug** | **String** |  | [required] |
**state** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Pullrequest>**](pullrequest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repositories_by_owner

> Vec<crate::models::Repository> get_repositories_by_owner(username)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Repository>**](repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repository

> crate::models::Repository get_repository(username, slug)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**slug** | **String** |  | [required] |

### Return type

[**crate::models::Repository**](repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_by_name

> crate::models::User get_user_by_name(username)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

[**crate::models::User**](user.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## merge_pull_request

> merge_pull_request(username, slug, pid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**slug** | **String** |  | [required] |
**pid** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

