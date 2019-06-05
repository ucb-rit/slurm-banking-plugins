# \UsersApi

All URIs are relative to *http://localhost:8880/mybrc-rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_list**](UsersApi.md#users_list) | **Get** /users/ | 
[**users_read**](UsersApi.md#users_read) | **Get** /users/{id}/ | 


# **users_list**
> ::models::InlineResponse2003 users_list(ctx, optional)


ViewSet for /api/users/

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

[**::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **users_read**
> ::models::User users_read(ctx, id)


Get one user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **i32**| A unique integer value identifying this SCGUser. | 

### Return type

[**::models::User**](User.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

