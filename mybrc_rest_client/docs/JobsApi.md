# \JobsApi

All URIs are relative to *http://scgup-dev.lbl.gov:8888/mybrc-rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**jobs_create**](JobsApi.md#jobs_create) | **Post** /jobs/ | 
[**jobs_update**](JobsApi.md#jobs_update) | **Put** /jobs/{jobslurmid}/ | 


# **jobs_create**
> ::models::Job jobs_create(ctx, data)


Creates a new Job identified by the given Slurm ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **data** | [**Job**](Job.md)|  | 

### Return type

[**::models::Job**](Job.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **jobs_update**
> ::models::Job jobs_update(ctx, jobslurmid, data)


Updates all fields of the Job identified by the given Slurm ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **jobslurmid** | **String**|  | 
  **data** | [**Job**](Job.md)|  | 

### Return type

[**::models::Job**](Job.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

