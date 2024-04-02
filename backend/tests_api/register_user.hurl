POST http://localhost:8080/api/auth/register
Content-Type: application/json
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:70.0) Gecko/20100101 Firefox/70.0
{
  "user_type": "{{user_type}}",
  "username": "{{username}}",
  "email": "{{email}}",
  "password": "{{password}}",
  "year_of_birth": {{year_of_birth}},
  "gender": "{{gender}}"
}
HTTP 200
[Asserts]
cookie "token[HttpOnly]" exists