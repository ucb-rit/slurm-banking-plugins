# \PartitionsApi

All URIs are relative to *http://scgup-dev.lbl.gov:8888/mybrc-rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**partitions_list**](PartitionsApi.md#partitions_list) | **Get** /partitions/ | 


# **partitions_list**
> ::models::InlineResponse2001 partitions_list(ctx, optional)


A ViewSet for the Partition model.

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

[**::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

