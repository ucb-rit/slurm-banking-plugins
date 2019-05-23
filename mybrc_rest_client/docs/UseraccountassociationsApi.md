# \UseraccountassociationsApi

All URIs are relative to *http://localhost:8181*

Method | HTTP request | Description
------------- | ------------- | -------------
[**useraccountassociations_list**](UseraccountassociationsApi.md#useraccountassociations_list) | **Get** /useraccountassociations/ | 
[**useraccountassociations_partial_update**](UseraccountassociationsApi.md#useraccountassociations_partial_update) | **Patch** /useraccountassociations/{id}/ | 
[**useraccountassociations_read**](UseraccountassociationsApi.md#useraccountassociations_read) | **Get** /useraccountassociations/{id}/ | 
[**useraccountassociations_update**](UseraccountassociationsApi.md#useraccountassociations_update) | **Put** /useraccountassociations/{id}/ | 


# **useraccountassociations_list**
> ::models::InlineResponse2002 useraccountassociations_list(ctx, optional)


ViewSet for /api/useraccountassociations/

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **page** | **i32**| A page number within the paginated result set. | 

### Return type

[**::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **useraccountassociations_partial_update**
> ::models::UserAccount useraccountassociations_partial_update(ctx, id, data)


ViewSet for /api/useraccountassociations/

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i32**| A unique integer value identifying this Active User Project Link. | 
  **data** | [**UserAccount**](UserAccount.md)|  | 

### Return type

[**::models::UserAccount**](UserAccount.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **useraccountassociations_read**
> ::models::UserAccount useraccountassociations_read(ctx, id)


Get one UserAccountAssociation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i32**| A unique integer value identifying this Active User Project Link. | 

### Return type

[**::models::UserAccount**](UserAccount.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **useraccountassociations_update**
> ::models::UserAccount useraccountassociations_update(ctx, id, data)


ViewSet for /api/useraccountassociations/

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i32**| A unique integer value identifying this Active User Project Link. | 
  **data** | [**UserAccount**](UserAccount.md)|  | 

### Return type

[**::models::UserAccount**](UserAccount.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

