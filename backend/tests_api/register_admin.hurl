POST http://localhost:8080/api/admin/auth/register
Content-Type: application/json
{
  "email": "{{email}}",
  "password": "{{password}}"
}
HTTP 200
[Asserts]
cookie "token[HttpOnly]" exists