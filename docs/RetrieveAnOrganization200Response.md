# RetrieveAnOrganization200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access** | **Vec<String>** |  | 
**allow_shared_issues** | **bool** |  | 
**available_roles** | [**Vec<crate::models::RetrieveAnOrganization200ResponseAvailableRolesInner>**](Retrieve_an_Organization_200_response_availableRoles_inner.md) |  | 
**avatar** | [**crate::models::ListAnOrganizationSTeams200ResponseInnerAvatar**](List_an_Organization_s_Teams_200_response_inner_avatar.md) |  | 
**data_scrubber** | **bool** |  | 
**data_scrubber_defaults** | **bool** |  | 
**date_created** | **String** |  | 
**default_role** | **String** |  | 
**enhanced_privacy** | **bool** |  | 
**experiments** | [**serde_json::Value**](.md) |  | 
**features** | **Vec<String>** |  | 
**id** | **String** |  | 
**is_default** | **bool** |  | 
**is_early_adopter** | **bool** |  | 
**name** | **String** |  | 
**onboarding_tasks** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | 
**open_membership** | **bool** |  | 
**pending_access_requests** | **i64** |  | 
**projects** | [**Vec<crate::models::RetrieveAnOrganization200ResponseProjectsInner>**](Retrieve_an_Organization_200_response_projects_inner.md) |  | 
**quota** | [**serde_json::Value**](.md) |  | 
**require2_fa** | **bool** |  | 
**safe_fields** | **Vec<String>** |  | 
**scrape_java_script** | **bool** |  | 
**scrub_ip_addresses** | **bool** |  | 
**sensitive_fields** | **Vec<String>** |  | 
**slug** | **String** |  | 
**status** | [**crate::models::RetrieveAnOrganization200ResponseAvailableRolesInner**](Retrieve_an_Organization_200_response_availableRoles_inner.md) |  | 
**store_crash_reports** | **i64** |  | 
**teams** | [**Vec<crate::models::CreateANewTeam201Response>**](Create_a_New_Team_201_response.md) |  | 
**trusted_relays** | **Vec<String>** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


