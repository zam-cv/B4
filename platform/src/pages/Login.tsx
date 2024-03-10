import { useRef } from "react";
import { useNavigate } from "react-router-dom";
import { SERVER_URL } from "../utils/constants";
import axios from "axios";

export default function Login() {
  const emailRef = useRef<HTMLInputElement>(null);
  const passwordRef = useRef<HTMLInputElement>(null);
  const buttonRef = useRef<HTMLButtonElement>(null);
  const navigate = useNavigate();

  function login() {
    const email = emailRef.current?.value;
    const password = passwordRef.current?.value;
    const button = buttonRef.current;

    if (!email || !password || !button) {
      return;
    }

    button.disabled = true;
    button.innerText = "Signing in...";

    axios
      .post(
        `${SERVER_URL}/api/admin/auth/signin`,
        {
          email,
          password,
        },
        { withCredentials: true }
      )
      .then((_) => {
        navigate("/dashboard");
      })
      .catch((error) => {
        button.disabled = false;
        button.innerText = "Sign in";
        console.error(error);
      });
  }

  return (
    <div className="flex flex-col items-center justify-center gap-5 h-full">
      <h1 className="text-3xl font-bold">Sign in</h1>
      <input
        ref={emailRef}
        type="email"
        className="rounded-md border border-gray-300 p-2"
      />
      <input
        ref={passwordRef}
        type="password"
        className="rounded-md border border-gray-300 p-2"
      />
      <button
        ref={buttonRef}
        onClick={login}
        className="rounded-md bg-blue-500 text-white p-2 px-10 hover:bg-blue-600"
      >
        Sign in
      </button>
    </div>
  );
}
