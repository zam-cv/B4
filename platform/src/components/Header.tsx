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
  const { signout, admin, permissions } = useAuth();

  return (
    <div className="grid grid-cols-[1fr_auto] p-1 shadow-lg px-10">
      <div className="flex">
        <div className="p-4 px-4 font-bold hover:underline cursor-pointer relative flex items-center justify-center w-[80px] mr-5">
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
      <div className="flex items-center">
        <span
          onClick={signout}
          className="p-4 px-4 font-semibold hover:underline cursor-pointer text-blue-900"
        >
          Cerrar Sesión
        </span>
        <div>
          <span className="p-2 px-3 font-bold uppercase rounded-full border-2 border-blue-900 text-blue-900">
            {admin?.email.charAt(0)}
          </span>
        </div>
      </div>
    </div>
  );
}
