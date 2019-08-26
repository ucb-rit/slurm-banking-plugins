# \UsersApi

All URIs are relative to *http://scgup-dev.lbl.gov:8888/mybrc-rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_list**](UsersApi.md#users_list) | **Get** /users/ | 


# **users_list**
> ::models::InlineResponse2002 users_list(ctx, optional)


A ViewSet for the SCGUser model.

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

