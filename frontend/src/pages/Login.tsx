import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { DotLottieReact } from '@lottiefiles/dotlottie-react';
import { authService } from "../services/authService";


export default function Login() {
  const navigate = useNavigate();

  const [usuario, setUsuario] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const [error, setError] = useState<string>("");
  const [loading, setLoading] = useState<boolean>(false);

  async function handleLogin(e: React.FormEvent) {
    e.preventDefault();
    setError("");
    
    if (!usuario || !password) {
      setError("Completa todos los campos");
      return;
    }

    setLoading(true);

    try {
      // endpoindcito
      
      sessionStorage.setItem("orbitlyUser", usuario);
      
      console.log("Login exitoso");
      
      navigate("/feed");
    } catch (err: any) {
      setError(err.message || "Error al iniciar sesión");
    } finally {
      setLoading(false);
    }
  }

  return (
      <div className="bg-gray-800 w-full h-full p-6">

        <div className="flex min-h-full flex-col justify-center px-6 py-6">

          {/* Logo */}
         <div className="sm:mx-auto sm:w-full sm:w-md md:w-lg lg:w-4xl">
            <DotLottieReact
              src="https://lottie.host/fed58175-554f-4a49-bac2-5aa9367e2ce0/vNm78QANu5.lottie"
              loop
              autoplay
            />
            <h2 className="mt-10 text-center text-2xl font-bold tracking-tight text-white">
              Inicia sesión en Orbitly
            </h2>
          </div>

          {/* Form */}
          <div className="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
            {error && <p className="text-red-400 text-center mb-4">{error}</p>}

            <form onSubmit={handleLogin} className="space-y-6">
              
              {/* Usuario */}
              <div>
                <label
                  htmlFor="usuario"
                  className="block text-sm font-medium text-gray-100"
                >
                  Usuario
                </label>
                <div className="mt-2">
                  <input
                    id="usuario"
                    type="text"
                    value={usuario}
                    onChange={(e) => setUsuario(e.target.value)}
                    disabled={loading}  
                    required
                    className="block w-full rounded-md bg-white/5 px-3 py-1.5 
                    text-base text-white outline-1 outline-white/10 
                    placeholder:text-gray-500 focus:outline-2 
                    focus:outline-indigo-500 disabled:opacity-50"
                  />
                </div>
              </div>

              {/* Password */}
              <div>
                <label
                  htmlFor="password"
                  className="block text-sm font-medium text-gray-100"
                >
                  Contraseña
                </label>

                <div className="mt-2">
                  <input
                    id="password"
                    type="password"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                    disabled={loading}  
                    required
                    className="block w-full rounded-md bg-white/5 px-3 py-1.5 
                    text-base text-white outline-1 outline-white/10 
                    placeholder:text-gray-500 focus:outline-2 
                    focus:outline-indigo-500 disabled:opacity-50"
                  />
                </div>
              </div>

              {/* Botón */}
              <div>
                <button
                  type="submit"
                  disabled={loading}  
                  className="flex w-full justify-center rounded-md bg-indigo-500 
                  px-3 py-1.5 text-sm font-semibold text-white 
                  hover:bg-indigo-400 focus-visible:outline-2 
                  focus-visible:outline-indigo-500 disabled:opacity-50 
                  disabled:cursor-not-allowed"
                >
                  {loading ? "Cargando..." : "Entrar"}  
                </button>
              </div>
            </form>

            {/* Registro */}
            <p className="mt-10 text-center text-sm text-gray-400">
              ¿No tienes cuenta?{" "}
              <a
                href="/register"
                className="font-semibold text-indigo-400 hover:text-indigo-300"
              >
                Crear cuenta
              </a>
            </p>
          </div>
        </div>

      </div>
  );
}