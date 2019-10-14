# \UploadCpuDataApi

All URIs are relative to *https://scgup-dev.lbl.gov/mybrc-rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**upload_cpu_data_update**](UploadCpuDataApi.md#upload_cpu_data_update) | **put** /upload_cpu_data/{filename} | 



## upload_cpu_data_update

> upload_cpu_data_update(filename, authorization)


Creates CPU data from the given zipped JSON file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filename** | **String** |  | Required | 
**authorization** | **String** | The authorization token for the requester. The token should be preceded by 'Token ' (no quotes). |  | 

### Return type

 (empty response body)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

