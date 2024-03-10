POST http://localhost:8080/api/auth/signin
Content-Type: application/json
{
  "email": "{{email}}",
  "password": "{{password}}"
}
HTTP 200
[Asserts]
jsonpath "$.message" == "Success"