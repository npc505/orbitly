import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { authService } from "../services/authService";

export default function Register() {
  const navigate = useNavigate();

  const [usuario, setUsuario] = useState("");
  const [correo, setCorreo] = useState("");
  const [password, setPassword] = useState("");
  const [password2, setPassword2] = useState("");
  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  // const [edad, setEdad] = useState("");
  // const [pais, setPais] = useState("");
  // const [avatar, setAvatar] = useState<string | null>(null);
  // const [descripcion, setDescripcion] = useState("");
  
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  // function handleAvatarUpload(e: React.ChangeEvent<HTMLInputElement>) {
  //   const file = e.target.files?.[0];
  //   if (!file) return;
  //   const reader = new FileReader();
  //   reader.onloadend = () => setAvatar(reader.result as string);
  //   reader.readAsDataURL(file);
  // }

  async function handleRegister(e: React.FormEvent) {
    e.preventDefault();
    setError("");

    if (!usuario || !correo || !password || !password2) {
      setError("Completa todos los campos obligatorios");
      return;
    }

    if (password !== password2) {
      setError("Las contraseñas no coinciden");
      return;
    }

    setLoading(true);

    try {
      await authService.register({
        username: usuario,
        mail: correo,
        password: password,
        password2: password2,
        first_name: firstName || undefined,
        last_name: lastName || undefined,
      });

      console.log("Registro exitoso");

      // login si sí jala
      navigate("/");
    } catch (err: any) {
      setError(err.message || "Error al registrar usuario");
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="bg-gray-800 w-full h-full p-6 text-white overflow-y-auto">

      <h2 className="text-center text-2xl font-bold">Crear tu cuenta</h2>
      <p className="text-center text-gray-300 text-sm mb-6">
        Bienvenida a Orbitly
      </p>

      {error && <p className="text-red-400 text-center mb-4">{error}</p>}

      <form onSubmit={handleRegister} className="space-y-5 max-w-md mx-auto">

        {/* Usuario */}
        <div>
          <label className="block text-sm font-medium">Usuario *</label>
          <input
            type="text"
            className="w-full bg-white/5 px-3 py-2 rounded-md text-white outline-none 
            focus:outline-2 focus:outline-indigo-500 disabled:opacity-50"
            value={usuario}
            onChange={(e) => setUsuario(e.target.value)}
            disabled={loading}
            required
          />
        </div>

        {/* Correo */}
        <div>
          <label className="block text-sm font-medium">Correo *</label>
          <input
            type="email"
            className="w-full bg-white/5 px-3 py-2 rounded-md text-white outline-none 
            focus:outline-2 focus:outline-indigo-500 disabled:opacity-50"
            value={correo}
            onChange={(e) => setCorreo(e.target.value)}
            disabled={loading}
            required
          />
        </div>

        {/* Nombre (opcional) */}
        <div>
          <label className="block text-sm font-medium">Nombre</label>
          <input
            type="text"
            className="w-full bg-white/5 px-3 py-2 rounded-md text-white outline-none 
            focus:outline-2 focus:outline-indigo-500 disabled:opacity-50"
            value={firstName}
            onChange={(e) => setFirstName(e.target.value)}
            disabled={loading}
          />
        </div>

        {/* Apellido (opcional) */}
        <div>
          <label className="block text-sm font-medium">Apellido</label>
          <input
            type="text"
            className="w-full bg-white/5 px-3 py-2 rounded-md text-white outline-none 
            focus:outline-2 focus:outline-indigo-500 disabled:opacity-50"
            value={lastName}
            onChange={(e) => setLastName(e.target.value)}
            disabled={loading}
          />
        </div>

        {/* Password */}
        <div>
          <label className="block text-sm font-medium">Contraseña *</label>
          <input
            type="password"
            className="w-full bg-white/5 px-3 py-2 rounded-md text-white outline-none 
            focus:outline-2 focus:outline-indigo-500 disabled:opacity-50"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            disabled={loading}
            required
          />
        </div>

        {/* Confirmar Password */}
        <div>
          <label className="block text-sm font-medium">Confirmar Contraseña *</label>
          <input
            type="password"
            className="w-full bg-white/5 px-3 py-2 rounded-md text-white outline-none 
            focus:outline-2 focus:outline-indigo-500 disabled:opacity-50"
            value={password2}
            onChange={(e) => setPassword2(e.target.value)}
            disabled={loading}
            required
          />
        </div>

        {/*
        <div>
          <label className="block text-sm font-medium">Edad</label>
          <input
            type="text"
            inputMode="numeric"
            className="w-full bg-white/5 px-3 py-2 rounded-md text-white outline-none"
            value={edad}
            onChange={(e) => {
              const v = e.target.value;
              if (/^\d*$/.test(v)) {
                setEdad(v);
              }
            }}
          />
        </div>
        */}

        {/*
        <div>
          <label className="block text-sm font-medium">País</label>
          <select
            className="w-full bg-white/5 px-3 py-2 rounded-md text-white outline-none"
            value={pais}
            onChange={(e) => setPais(e.target.value)}
          >
            <option value="">Selecciona tu país</option>
            <option value="México">🇲🇽 México</option>
            <option value="Estados Unidos">🇺🇸 Estados Unidos</option>
            <option value="España">🇪🇸 España</option>
            <option value="Argentina">🇦🇷 Argentina</option>
            <option value="Colombia">🇨🇴 Colombia</option>
            <option value="Chile">🇨🇱 Chile</option>
            <option value="Japón">🇯🇵 Japón</option>
            <option value="Otro">🌍 Otro</option>
          </select>
        </div>
        */}

        {/*
        <div className="flex flex-col items-center">
          <label className="block text-sm font-medium mb-2">Foto de perfil</label>

          <div
            className="w-24 h-24 rounded-full bg-white/10 border border-white/20 
            flex items-center justify-center overflow-hidden cursor-pointer"
            onClick={() => document.getElementById("avatarInput")?.click()}
          >
            {avatar ? (
              <img src={avatar} className="w-full h-full object-cover" />
            ) : (
              <span className="text-gray-400 text-sm text-center">Tocar para subir</span>
            )}
          </div>

          <input
            id="avatarInput"
            type="file"
            accept="image/*"
            className="hidden"
            onChange={handleAvatarUpload}
          />

          <button
            type="button"
            onClick={() => document.getElementById("avatarInput")?.click()}
            className="mt-3 bg-indigo-500 hover:bg-indigo-400 px-4 py-1.5 rounded-md text-sm font-medium"
          >
            Elegir foto
          </button>
        </div>
        */}

        {/*
        <div>
          <label className="block text-sm font-medium">Descripción</label>
          <textarea
            className="w-full bg-white/5 px-3 py-2 rounded-md text-white outline-none h-24 resize-none"
            value={descripcion}
            onChange={(e) => setDescripcion(e.target.value)}
          />
        </div>
        */}

        {/* Botón */}
        <button
          type="submit"
          disabled={loading}
          className="w-full bg-indigo-500 py-2 rounded-md font-semibold hover:bg-indigo-400 
          disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {loading ? "Creando cuenta..." : "Crear cuenta"}
        </button>

        <p className="text-center text-sm mt-4 text-gray-300">
          ¿Ya tienes cuenta?{" "}
          <span
            className="text-indigo-400 pb-8 font-semibold hover:text-indigo-300 cursor-pointer"
            onClick={() => navigate("/")}
          >
            Inicia sesión
          </span>
        </p>
      </form>
    </div>
  );
}