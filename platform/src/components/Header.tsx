import { useState, useEffect } from "react";
import { Link } from "react-router-dom";
import { useAuth } from "../hooks/useAuth";
import { pagePermissions } from "../App";
import logo from "../assets/verqor.svg";

export function Sub({ title, route }: { title: string; route: string }) {
  return (
    <Link to={route}>
      <div className="p-4 px-4 hover:underline font-semibold cursor-pointer select-none text-blue-900">
        {title}
      </div>
    </Link>
  );
}

export default function Header() {
  const [open, setOpen] = useState(true);
  const { signout, admin, permissions } = useAuth();

  useEffect(() => {
    if (!open) {
      document.body.classList.add("dark-theme");
    } else {
      document.body.classList.remove("dark-theme");
    }
  }, [open]);

  return (
    <div className="grid grid-cols-[1fr_auto] p-1 shadow-lg px-10">
      <div className="flex">
        <div className="p-4 font-bold hover:underline cursor-pointer relative flex items-center justify-center w-[80px] mr-5">
          <img src={logo} alt="Verqor" className="w-full h-full absolute" />
        </div>
        {pagePermissions.map((page) => {
          if (permissions?.has(page.permission)) {
            return (
              <Sub key={page.title} title={page.title} route={page.route} />
            );
          }
          return null;
        })}
      </div>
      <div className="flex items-center gap-5">
        <span
          onClick={signout}
          className="font-semibold hover:underline cursor-pointer text-blue-900"
        >
          Cerrar Sesión
        </span>
        <div>
          <span onClick={() => setOpen(!open)}>
            {open ? (
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="currentColor"
                className="w-6 h-6 cursor-pointer text-blue-900 select-none"
              >
                <path
                  fillRule="evenodd"
                  d="M9.528 1.718a.75.75 0 0 1 .162.819A8.97 8.97 0 0 0 9 6a9 9 0 0 0 9 9 8.97 8.97 0 0 0 3.463-.69.75.75 0 0 1 .981.98 10.503 10.503 0 0 1-9.694 6.46c-5.799 0-10.5-4.7-10.5-10.5 0-4.368 2.667-8.112 6.46-9.694a.75.75 0 0 1 .818.162Z"
                  clipRule="evenodd"
                />
              </svg>
            ) : (
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="currentColor"
                className="w-6 h-6 cursor-pointer text-blue-900 select-none"
              >
                <path d="M12 2.25a.75.75 0 0 1 .75.75v2.25a.75.75 0 0 1-1.5 0V3a.75.75 0 0 1 .75-.75ZM7.5 12a4.5 4.5 0 1 1 9 0 4.5 4.5 0 0 1-9 0ZM18.894 6.166a.75.75 0 0 0-1.06-1.06l-1.591 1.59a.75.75 0 1 0 1.06 1.061l1.591-1.59ZM21.75 12a.75.75 0 0 1-.75.75h-2.25a.75.75 0 0 1 0-1.5H21a.75.75 0 0 1 .75.75ZM17.834 18.894a.75.75 0 0 0 1.06-1.06l-1.59-1.591a.75.75 0 1 0-1.061 1.06l1.59 1.591ZM12 18a.75.75 0 0 1 .75.75V21a.75.75 0 0 1-1.5 0v-2.25A.75.75 0 0 1 12 18ZM7.758 17.303a.75.75 0 0 0-1.061-1.06l-1.591 1.59a.75.75 0 0 0 1.06 1.061l1.591-1.59ZM6 12a.75.75 0 0 1-.75.75H3a.75.75 0 0 1 0-1.5h2.25A.75.75 0 0 1 6 12ZM6.697 7.757a.75.75 0 0 0 1.06-1.06l-1.59-1.591a.75.75 0 0 0-1.061 1.06l1.59 1.591Z" />
              </svg>
            )}
          </span>
        </div>
        <div>
          <span className="p-2 px-3 font-bold uppercase rounded-full border-2 border-blue-900 text-blue-900">
            {admin?.email.charAt(0)}
          </span>
        </div>
      </div>
    </div>
  );
}
