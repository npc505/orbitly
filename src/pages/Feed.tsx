import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

const defaultAvatar =
  "https://upload.wikimedia.org/wikipedia/commons/thumb/5/59/User-avatar.svg/2048px-User-avatar.svg.png";

interface MatchUser {
  id: string;
  usuario: string;
  avatar: string;
  compatibilidad: number;
}

interface Chat {
  id: string;
  usuario: string;
  avatar: string;
  mensaje: string;
  time: string;
}

interface Sugerido {
  id: string;
  usuario: string;
  avatar: string;
  compatibilidad: number;
}

export default function Feed() {
  const navigate = useNavigate();
  const usuario = "ana_g"; // u1

  const [matches, setMatches] = useState<MatchUser[]>([]);
  const [chats, setChats] = useState<Chat[]>([]);
  const [sugeridos, setSugeridos] = useState<Sugerido[]>([]);

  useEffect(() => {
    // MATCHES
    setMatches([
      {
        id: "u6",
        usuario: "marcodz",
        avatar: defaultAvatar,
        compatibilidad: 76
      },
      {
        id: "u5",
        usuario: "sof_lo",
        avatar: defaultAvatar,
        compatibilidad: 62
      },
      {
        id: "u7",
        usuario: "elena_ct",
        avatar: defaultAvatar,
        compatibilidad: 55
      }
    ]);

    // CHATS
    setChats([
      {
        id: "u2",
        usuario: "luism_dev",
        avatar: defaultAvatar,
        mensaje: "¿Te gustó Elden Ring?",
        time: "2h"
      },
      {
        id: "u5",
        usuario: "sof_lo",
        avatar: defaultAvatar,
        mensaje: "Hola! ¿te gusta Ghibli?",
        time: "6h"
      }
    ]);

    // SUGERIDOS
    setSugeridos([
      { id: "u3", usuario: "carla_r", avatar: defaultAvatar, compatibilidad: 48 },
      { id: "u2", usuario: "luism_dev", avatar: defaultAvatar, compatibilidad: 41 },
      { id: "u9", usuario: "valtor", avatar: defaultAvatar, compatibilidad: 33 }
    ]);
  }, []);

  return (
    <div className="bg-gray-800 text-white w-full h-full p-5 overflow-y-auto">

      {/* Header */}
      <h2 className="text-xl font-bold mb-4">Bienvenida, Ana</h2>

      {/* MATCHES */}
      <div>
        <h3 className="text-lg font-semibold mb-2">Tus Matches</h3>

        <div className="flex space-x-4 overflow-x-auto pb-3">

          {matches.map(m => (
            <div key={m.id} className="flex flex-col items-center">
              <img
                src={m.avatar}
                className="w-20 h-20 rounded-full bg-gray-700 object-cover border border-pink-400"
              />
              <p className="mt-2 font-medium">@{m.usuario}</p>
              <p className="text-xs text-gray-300">
                Compatibilidad:{" "}
                <span className="text-pink-300 font-bold">{m.compatibilidad}%</span>
              </p>
            </div>
          ))}

        </div>
      </div>

      {/* CHATS */}
      <div className="mt-6">
        <h3 className="text-lg font-semibold mb-2">💬 Chats recientes</h3>

        <div className="space-y-3">
          {chats.map(c => (
            <div
              key={c.id}
              className="flex items-center bg-gray-800 p-3 rounded-xl cursor-pointer hover:bg-gray-700"
              onClick={() => navigate(`/chat/${c.id}`)}
            >
              <img
                src={c.avatar}
                className="w-14 h-14 rounded-full bg-gray-700 object-cover mr-3"
              />

              <div className="flex-1">
                <p className="font-semibold">@{c.usuario}</p>
                <p className="text-sm text-gray-400 truncate">{c.mensaje}</p>
              </div>

              <span className="text-xs text-gray-500">{c.time}</span>
            </div>
          ))}
        </div>
      </div>

      {/* SUGERIDOS */}
      <div className="mt-8 pb-10">
        <h3 className="text-lg font-semibold mb-2">Personas sugeridas</h3>

        <div className="space-y-4">
          {sugeridos.map(s => (
            <div
              key={s.id}
              className="flex items-center bg-gray-800 p-3 rounded-xl hover:bg-gray-700"
            >
              <img
                src={s.avatar}
                className="w-16 h-16 rounded-full bg-gray-700 object-cover mr-3"
              />

              <div className="flex-1">
                <p className="font-semibold">@{s.usuario}</p>
                <p className="text-sm text-gray-300">
                  Compatibilidad:{" "}
                  <span className="font-bold text-purple-300">
                    {s.compatibilidad}%
                  </span>
                </p>
              </div>

              <button className="bg-indigo-500 px-3 py-1.5 rounded-md text-sm">
                Conectar
              </button>
            </div>
          ))}
        </div>
      </div>

    </div>
  );
}
