import { useState } from "react";
import { useNavigate } from "react-router-dom";

export default function Register() {
  const navigate = useNavigate();

  const [usuario, setUsuario] = useState("");
  const [correo, setCorreo] = useState("");
  const [password, setPassword] = useState("");
  const [edad, setEdad] = useState("");
  const [pais, setPais] = useState("");
  const [avatar, setAvatar] = useState<string | null>(null);
  const [descripcion, setDescripcion] = useState("");
  const [error, setError] = useState("");

  // Subir foto (convertida a Base64)
  function handleAvatarUpload(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onloadend = () => setAvatar(reader.result as string);
    reader.readAsDataURL(file);
  }

  // Guardar temporalmente
  function handleRegister(e: React.FormEvent) {
    e.preventDefault();
    setError("");

    if (!usuario || !correo || !password || !edad || !pais || !avatar || !descripcion) {
      setError("Completa todos los campos");
      return;
    }

    const newUser = {
      usuario,
      correo,
      hash_contrasena: password,
      edad: Number(edad),
      pais,
      avatar,
      descripcion,
    };

    localStorage.setItem("orbitlyNewUser", JSON.stringify(newUser));

    navigate("/");
  }

  return (
    <div className="bg-gray-800 w-full h-full p-6 text-white">

      <h2 className="text-center text-2xl font-bold">Crear tu cuenta</h2>
      <p className="text-center text-gray-300 text-sm mb-6">
        Bienvenida a Orbitly
      </p>

      {error && <p className="text-red-400 text-center mb-4">{error}</p>}

      <form onSubmit={handleRegister} className="space-y-5">

        {/* Usuario */}
        <div>
          <label className="block text-sm font-medium">Usuario</label>
          <input
            type="text"
            className="w-full bg-white/5 px-3 py-2 rounded-md text-white outline-none"
            value={usuario}
            onChange={(e) => setUsuario(e.target.value)}
          />
        </div>

        {/* Correo */}
        <div>
          <label className="block text-sm font-medium">Correo</label>
          <input
            type="email"
            className="w-full bg-white/5 px-3 py-2 rounded-md text-white outline-none"
            value={correo}
            onChange={(e) => setCorreo(e.target.value)}
          />
        </div>

        {/* Password */}
        <div>
          <label className="block text-sm font-medium">Contraseña</label>
          <input
            type="password"
            className="w-full bg-white/5 px-3 py-2 rounded-md text-white outline-none"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
          />
        </div>

        {/* Edad */}
        <div>
          <label className="block text-sm font-medium">Edad</label>
          <input
            type="text"
            inputMode="numeric"
            className="w-full bg-white/5 px-3 py-2 rounded-md text-white outline-none"
            value={edad}
            onChange={(e) => {
              const v = e.target.value;
              if (/^\d*$/.test(v)) { // Solo números
                setEdad(v);
              }
            }}
          />
        </div>

        {/* País */}
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

        {/* Descripción */}
        <div>
          <label className="block text-sm font-medium">Descripción</label>
          <textarea
            className="w-full bg-white/5 px-3 py-2 rounded-md text-white outline-none h-24 resize-none"
            value={descripcion}
            onChange={(e) => setDescripcion(e.target.value)}
          />
        </div>

        {/* Botón */}
        <button
          type="submit"
          className="w-full bg-indigo-500 py-2 rounded-md font-semibold hover:bg-indigo-400"
        >
          Crear cuenta
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
