import { handleKeyDown, handleEnter } from "../utils";
import { useRef, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { useAuth } from "../hooks/useAuth";

export default function Login() {
  const { isAuthenticated, signin, loading } = useAuth();
  const emailRef = useRef<HTMLInputElement>(null);
  const passwordRef = useRef<HTMLInputElement>(null);
  const buttonRef = useRef<HTMLButtonElement>(null);
  const navigate = useNavigate();

  function login() {
    const email = emailRef.current?.value;
    const password = passwordRef.current?.value;

    if (!email || !password) return;
    signin(email, password);
  }

  useEffect(() => {
    let button = buttonRef.current;
    if (!button) return;

    if (loading) {
      button.disabled = true;
      button.innerText = "Signing in...";
    } else {
      button.disabled = false;
      button.innerText = "Sign in";
    }
  }, [loading, isAuthenticated]);

  if (isAuthenticated) {
    navigate("/");
  }

  return (
    <div className="flex flex-col items-center justify-center gap-5 h-full select-none">
      <h1 className="text-3xl font-bold">Sign in</h1>
      <input
        ref={emailRef}
        type="email"
        onKeyDown={(e) => handleKeyDown(e, passwordRef)}
        tabIndex={1}
        className="rounded-md border border-gray-300 p-2"
      />
      <input
        ref={passwordRef}
        type="password"
        onKeyDown={(e) => handleEnter(e, login)}
        tabIndex={2}
        className="rounded-md border border-gray-300 p-2"
      />
      <button
        ref={buttonRef}
        onClick={login}
        tabIndex={3}
        className="rounded-md bg-blue-500 text-white p-2 px-10 hover:bg-blue-600"
      >
        Sign in
      </button>
    </div>
  );
}
