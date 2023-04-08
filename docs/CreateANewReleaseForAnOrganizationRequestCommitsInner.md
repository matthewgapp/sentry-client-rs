# CreateANewReleaseForAnOrganizationRequestCommitsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**patch_set** | Option<[**Vec<crate::models::CreateANewReleaseForAnOrganizationRequestCommitsInnerPatchSetInner>**](Create_a_New_Release_for_an_Organization_request_commits_inner_patch_set_inner.md)> | A list of the files that have been changed in the commit. Specifying the patch_set is necessary to power suspect commits and suggested assignees. | [optional]
**repository** | Option<**String**> | The full name of the repository the commit belongs to. If this field is not given Sentry will generate a name in the form: u'organization-<organization_id>' (i.e. if the organization id is 123, then the generated repository name will be u'organization-123). | [optional]
**author_name** | Option<**String**> | The name of the commit author. | [optional]
**author_email** | Option<**String**> | The email of the commit author. The commit author's email is required to enable the suggested assignee feature. | [optional]
**timestamp** | Option<**String**> | The commit timestamp is used to sort the commits given. If a timestamp is not included, the commits will remain sorted in the order given. | [optional]
**message** | Option<**String**> | The commit message. | [optional]
**id** | Option<**String**> | The commit ID (the commit SHA). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


