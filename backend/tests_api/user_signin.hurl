POST http://localhost:8080/api/auth/signin
Content-Type: application/json
{
  "username": "{{username}}",
  "password": "{{password}}"
}
HTTP 200
[Asserts]
cookie "token[HttpOnly]" exists
