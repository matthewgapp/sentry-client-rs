# UpdateAnOrganizationSReleaseRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#ref** | Option<**String**> | An optional commit reference. This is useful if a tagged version has been provided. | [optional]
**url** | Option<**String**> | A URL that points to the release. This can be the path to an online interface to the source code for instance. | [optional]
**date_released** | Option<**String**> | An optional date that indicates when the release went live. If not provided the current time is assumed. | [optional]
**commits** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | An optional list of commit data to be associated with the release. Commits must include parameters `id` (the sha of the commit), and can optionally include `repository`, `message`, `author_name`, `author_email`, and `timestamp`. | [optional]
**refs** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | An optional way to indicate the start and end commits for each repository included in a release. Head commits must include parameters `repository` and `commit` (the HEAD sha). They can optionally include `previousCommit` (the sha of the HEAD of the previous release), which should be specified if this is the first time you've sent commit data. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


