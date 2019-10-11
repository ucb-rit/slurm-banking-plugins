# \JobsApi

All URIs are relative to *https://scgup-dev.lbl.gov/mybrc-rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**jobs_create**](JobsApi.md#jobs_create) | **post** /jobs/ | 
[**jobs_list**](JobsApi.md#jobs_list) | **get** /jobs/ | 
[**jobs_update**](JobsApi.md#jobs_update) | **put** /jobs/{jobslurmid}/ | 



## jobs_create

> crate::models::Job jobs_create(data, authorization)


Creates a new Job identified by the given Slurm ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**Job**](Job.md) |  | Required | 
**authorization** | **String** | The authorization token for the requester. The token should be preceded by 'Token ' (no quotes). |  | 

### Return type

[**crate::models::Job**](Job.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_list

> crate::models::InlineResponse2001 jobs_list(page)


A ViewSet for the Job model, intended for allocation accounting purposes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | **i32** | A page number within the paginated result set. |  | 

### Return type

[**crate::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_update

> crate::models::Job jobs_update(jobslurmid, data, authorization)


Updates all fields of the Job identified by the given Slurm ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobslurmid** | **String** |  | Required | 
**data** | [**Job**](Job.md) |  | Required | 
**authorization** | **String** | The authorization token for the requester. The token should be preceded by 'Token ' (no quotes). |  | 

### Return type

[**crate::models::Job**](Job.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

