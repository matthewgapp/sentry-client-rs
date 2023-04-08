# BulkMutateAListOfIssuesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | The new status for the issues. Valid values are `\"resolved\"`, `\"resolvedInNextRelease\"`, `\"unresolved\"`, and `\"ignored\"`. | [optional]
**status_details** | Option<[**crate::models::BulkMutateAListOfIssuesRequestStatusDetails**](Bulk_Mutate_a_List_of_Issues_request_statusDetails.md)> |  | [optional]
**ignore_duration** | Option<**i32**> | The number of minutes to ignore this issue. | [optional]
**is_public** | Option<**bool**> | Sets the issue to public or private. | [optional]
**merge** | Option<**bool**> | Allows to merge or unmerge different issues. | [optional]
**assigned_to** | Option<**String**> | The actor id (or username) of the user or team that should be assigned to this issue. | [optional]
**has_seen** | Option<**bool**> | In case this API call is invoked with a user context this allows changing of the flag that indicates if the user has seen the event. | [optional]
**is_bookmarked** | Option<**bool**> | In case this API call is invoked with a user context this allows changing of the bookmark flag. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


