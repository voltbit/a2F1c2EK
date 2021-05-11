The clients package contains clients taht will use the controller service.

Endpoints:

`POST /job`: create a new job

```
{
  jobID
  userID
  urgency
  protocol: (jdbc|odbc|s3|looker)
  query
}
```

`GET /status/jobID`: request status about a job

```
{
  status: (successful|pending|failed)
}
```
