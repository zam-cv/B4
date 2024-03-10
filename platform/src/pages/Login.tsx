import { useNavigate } from 'react-router-dom';

export default function Login() {
  const navigate = useNavigate();

  function login() {
    navigate('/dashboard');
  }

  return (
    <div className="flex flex-col items-center justify-center gap-5 h-full">
      <h1 className="text-3xl font-bold">Sign in</h1>
      <input type="email" className="rounded-md border border-gray-300 p-2" />
      <input
        type="password"
        className="rounded-md border border-gray-300 p-2"
      />
      <button
        onClick={login}
        className="rounded-md bg-blue-500 text-white p-2 px-10 hover:bg-blue-600"
      >
        Sign in
      </button>
    </div>
  );
}
