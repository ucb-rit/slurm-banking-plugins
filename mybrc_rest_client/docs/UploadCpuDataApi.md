# \UploadCpuDataApi

All URIs are relative to *https://scgup-dev.lbl.gov/mybrc-rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**upload_cpu_data_update**](UploadCpuDataApi.md#upload_cpu_data_update) | **Put** /upload_cpu_data/{filename} | 


# **upload_cpu_data_update**
> upload_cpu_data_update(ctx, filename, optional)


Creates CPU data from the given zipped JSON file.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **filename** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **filename** | **String**|  | 
 **authorization** | **String**| The authorization token for the requester. The token should be preceded by &#39;Token &#39; (no quotes). | 

### Return type

 (empty response body)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

