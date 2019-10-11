# \CanSubmitJobApi

All URIs are relative to *https://scgup-dev.lbl.gov/mybrc-rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**can_submit_job_read**](CanSubmitJobApi.md#can_submit_job_read) | **get** /can_submit_job/{job_cost}/{user_id}/{account_id}/ | 



## can_submit_job_read

> crate::models::InlineResponse200 can_submit_job_read(job_cost, user_id, account_id, authorization)


Returns whether or not a Job with the given cost can be submitted by the given user for the given account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_cost** | **String** | A string representation of a nonnegative decimal number with no greater than 11 total digits and no greater than 2 decimal places. | Required | 
**user_id** | **String** | A string representation of the user's cluster UID, a five digit number. | Required | 
**account_id** | **String** | The name of the account. | Required | 
**authorization** | **String** | The authorization token for the requester. The token should be preceded by 'Token ' (no quotes). |  | 

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

[Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

