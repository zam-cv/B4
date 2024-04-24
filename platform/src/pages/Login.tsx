import { handleKeyDown, handleEnter } from "../utils";
import { useRef, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { useAuth } from "../hooks/useAuth";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";

export default function Login() {
  const [serverHost, setServerHost] = useState("");
  const { isAuthenticated, signin, loading } = useAuth();
  const emailRef = useRef<HTMLInputElement>(null);
  const passwordRef = useRef<HTMLInputElement>(null);
  const buttonRef = useRef<HTMLButtonElement>(null);
  const navigate = useNavigate();

  function login() {
    const email = emailRef.current?.value;
    const password = passwordRef.current?.value;

    if (!email || !password) return;
    signin(email, password, serverHost);
  }

  useEffect(() => {
    let button = buttonRef.current;
    if (!button) return;

    if (loading) {
      button.disabled = true;
      button.innerText = "Iniciando Sesión...";
    } else {
      button.disabled = false;
      button.innerText = "Iniciar Sesión";
    }
  }, [loading, isAuthenticated]);

  if (isAuthenticated) {
    navigate("/");
  }

  return (
    <div className="w-full h-full flex items-center justify-center">
      <div className="container">
        <div className="grid grid-cols-2 max-md:grid-cols-1 shadow-lg rounded-2xl">
          <div className="flex flex-col gap-5 px-10 py-16">
            <div className="mb-5">
              <h1 className="text-2xl font-bold text-orange-500 mb-3">
                Iniciar Sesión
              </h1>
              <h2 className="text-lg font-semibold">Bienvenido!</h2>
              <p className="text-sm text-gray-500">
                Ingresa tu correo y contraseña para acceder a la plataforma
              </p>
            </div>
            <div className="flex flex-col gap-3">
              <div className="flex flex-col gap-8">
                <Input
                  ref={emailRef}
                  type="email"
                  placeholder="Correo electrónico"
                  onKeyDown={(e) => handleKeyDown(e, passwordRef)}
                />
                <Input
                  ref={passwordRef}
                  type="password"
                  onKeyDown={(e) => handleEnter(e, login)}
                  placeholder="Contraseña"
                />
              </div>
              <Accordion
                type="single"
                collapsible
                className="px-2 pt-3 pb-3 text-gray-500"
              >
                <AccordionItem className=" border-b-0" value="item-1">
                  <AccordionTrigger className="p-0 hover:no-underline">
                    Opciones del servidor
                  </AccordionTrigger>
                  <AccordionContent>
                    <div className="pt-3 px-3">
                      <Input
                        type="text"
                        placeholder="https://hostname:port"
                        value={serverHost}
                        onChange={(e) => setServerHost(e.target.value)}
                      />
                    </div>
                  </AccordionContent>
                </AccordionItem>
              </Accordion>
              <Button
                className="bg-orange-500 text-white hover:bg-orange-600 font-bold"
                ref={buttonRef}
                onClick={login}
              >
                Iniciar Sesión
              </Button>
            </div>
          </div>
          <div className="bg-gradient-to-r from-orange-600 to-orange-400 rounded-2xl flex items-end max-md:hidden">
            <div className="p-10 flex flex-col gap-5">
              <h2 className="text-2xl font-bold text-white text-center">
                Verqor te da la bienvenida
              </h2>
              <p className="text-white text-lg font-medium text-center">
                ¿Sabías que? Algunas plantas se comunican bajo tierra mediante
                señales químicas, activando defensas ante el estrés como la
                presencia de herbívoros.
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
