# \JobsApi

All URIs are relative to *https://scgup-dev.lbl.gov/mybrc-rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**jobs_create**](JobsApi.md#jobs_create) | **Post** /jobs/ | 
[**jobs_list**](JobsApi.md#jobs_list) | **Get** /jobs/ | 
[**jobs_update**](JobsApi.md#jobs_update) | **Put** /jobs/{jobslurmid}/ | 


# **jobs_create**
> ::models::Job jobs_create(ctx, data, optional)


Creates a new Job identified by the given Slurm ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **data** | [**Job**](Job.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **data** | [**Job**](Job.md)|  | 
 **authorization** | **String**| The authorization token for the requester. The token should be preceded by &#39;Token &#39; (no quotes). | 

### Return type

[**::models::Job**](Job.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **jobs_list**
> ::models::InlineResponse2001 jobs_list(ctx, optional)


A ViewSet for the Job model, intended for allocation accounting purposes.

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

# **jobs_update**
> ::models::Job jobs_update(ctx, jobslurmid, data, optional)


Updates all fields of the Job identified by the given Slurm ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **jobslurmid** | **String**|  | 
  **data** | [**Job**](Job.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **jobslurmid** | **String**|  | 
 **data** | [**Job**](Job.md)|  | 
 **authorization** | **String**| The authorization token for the requester. The token should be preceded by &#39;Token &#39; (no quotes). | 

### Return type

[**::models::Job**](Job.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

