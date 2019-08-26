# \CpusApi

All URIs are relative to *http://scgup-dev.lbl.gov:8888/mybrc-rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cpus_create**](CpusApi.md#cpus_create) | **Post** /cpus/ | 


# **cpus_create**
> ::models::Cpu cpus_create(ctx, data)


The method for POST (create) requests.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **data** | [**Cpu**](Cpu.md)|  | 

### Return type

[**::models::Cpu**](CPU.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

