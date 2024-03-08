POST http://localhost:8080/api/admin/auth/signin
Content-Type: application/json
{
  "username": "{{username}}",
  "password": "{{password}}"
}
HTTP 200
[Asserts]
jsonpath "$.message" == "Success"
[Captures]
token: jsonpath "$.payload.token"

GET http://localhost:8080/api/admin/users/statistics/{{id}}
Authorization: {{token}}
HTTP 200