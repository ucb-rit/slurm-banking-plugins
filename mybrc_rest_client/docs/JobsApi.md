# \JobsApi

All URIs are relative to *http://localhost:8181*

Method | HTTP request | Description
------------- | ------------- | -------------
[**jobs_create**](JobsApi.md#jobs_create) | **Post** /jobs/ | 
[**jobs_delete**](JobsApi.md#jobs_delete) | **Delete** /jobs/{jobnumber}/ | 
[**jobs_list**](JobsApi.md#jobs_list) | **Get** /jobs/ | 
[**jobs_partial_update**](JobsApi.md#jobs_partial_update) | **Patch** /jobs/{jobnumber}/ | 
[**jobs_read**](JobsApi.md#jobs_read) | **Get** /jobs/{jobnumber}/ | 
[**jobs_update**](JobsApi.md#jobs_update) | **Put** /jobs/{jobnumber}/ | 


# **jobs_create**
> ::models::Job jobs_create(ctx, data)


ViewSet for /api/jobs/

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

# **jobs_delete**
> jobs_delete(ctx, jobnumber)


ViewSet for /api/jobs/

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **jobnumber** | **i32**| A unique integer value identifying this job. | 

### Return type

 (empty response body)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **jobs_list**
> ::models::InlineResponse2001 jobs_list(ctx, optional)


List jobs.

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

# **jobs_partial_update**
> ::models::Job jobs_partial_update(ctx, jobnumber, data)


ViewSet for /api/jobs/

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **jobnumber** | **i32**| A unique integer value identifying this job. | 
  **data** | [**Job**](Job.md)|  | 

### Return type

[**::models::Job**](Job.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **jobs_read**
> ::models::Job jobs_read(ctx, jobnumber)


Get one Job.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **jobnumber** | **i32**| A unique integer value identifying this job. | 

### Return type

[**::models::Job**](Job.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **jobs_update**
> ::models::Job jobs_update(ctx, jobnumber, data)


ViewSet for /api/jobs/

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **jobnumber** | **i32**| A unique integer value identifying this job. | 
  **data** | [**Job**](Job.md)|  | 

### Return type

[**::models::Job**](Job.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

