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

POST http://localhost:8080/api/admin/data/crops
Content-Type: application/json
Authorization: {{token}}
{
  "name": "{{name}}",
  "price": {{price}}
}
HTTP 200
[Asserts]
jsonpath "$.message" == "Success"