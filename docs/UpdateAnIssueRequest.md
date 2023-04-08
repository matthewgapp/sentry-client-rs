# UpdateAnIssueRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | The new status for the issues. Valid values are `\"resolved\"`, `\"resolvedInNextRelease\"`, `\"unresolved\"`, and `\"ignored\"`. | [optional]
**assigned_to** | Option<**String**> | The actor id (or username) of the user or team that should be assigned to this issue. | [optional]
**has_seen** | Option<**bool**> | In case this API call is invoked with a user context this allows changing of the flag that indicates if the user has seen the event. | [optional]
**is_bookmarked** | Option<**bool**> | In case this API call is invoked with a user context this allows changing of the bookmark flag. | [optional]
**is_subscribed** | Option<**bool**> |  | [optional]
**is_public** | Option<**bool**> | Sets the issue to public or private. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


