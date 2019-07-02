# \CanSubmitJobApi

All URIs are relative to *http://localhost:8880/mybrc-rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**can_submit_job_read**](CanSubmitJobApi.md#can_submit_job_read) | **Get** /can_submit_job/{job_cost}/{user_id}/{account_id}/ | 


# **can_submit_job_read**
> ::models::InlineResponse200 can_submit_job_read(ctx, job_cost, user_id, account_id)


Returns whether or not a Job with the given cost can be submitted by the given user for the given account.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **job_cost** | **String**| A string representation of a nonnegative decimal number with no greater than 11 total digits and no greater than 2 decimal places. | 
  **user_id** | **String**| A string representation of the user&#39;s cluster UID, a five digit number. | 
  **account_id** | **String**| The name of the account. | 

### Return type

[**::models::InlineResponse200**](inline_response_200.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

