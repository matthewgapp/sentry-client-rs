# RetrieveReleaseHealthSessionStatistics200ResponseGroupsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**by** | [**serde_json::Value**](.md) | These are key/value pairs, the key being the requested `groupBy` property with its corresponding value. | 
**totals** | [**serde_json::Value**](.md) | These are key/value pairs, the key being the requested `field`, and the value the corresponding total over the requested time frame. | 
**series** | [**::std::collections::HashMap<String, Vec<i32>>**](array.md) | These are key/value pairs, the key being the requested `field`, and the value is an array of aggregated values. The array corresponds to the times given in the `intervals` array. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


