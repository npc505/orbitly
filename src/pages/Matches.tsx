import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { apiService } from "../services/apiService";

const defaultAvatar =
  "https://upload.wikimedia.org/wikipedia/commons/thumb/5/59/User-avatar.svg/2048px-User-avatar.svg.png";

interface MatchUser {
  id: string;
  usuario: string;
  avatar: string;
  compatibilidad: number;
}

export default function Matches() {
  const navigate = useNavigate();
  const [matches, setMatches] = useState<MatchUser[]>([]);
  const [recommended, setRecommended] = useState<MatchUser[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadMatches();
  }, []);

  async function loadMatches() {
    try {
      setLoading(true);

      // Cargar mis matches
      const matchesData = await apiService.getMyMatches();
      const formattedMatches = matchesData.map((match, index) => {
        const username = typeof match === 'string' ? match : match.username;
        const compatibility = typeof match === 'object' && match.compatibility 
        ? Math.round(match.compatibility * 100)
        : Math.floor(Math.random() * 40) + 60;
        
        return {
          id: `match_${index}`,
          usuario: username,
          avatar: match.avatar || defaultAvatar,
          compatibilidad: compatibility,
        };
      });
      setMatches(formattedMatches);

      // Cargar personas recomendadas (sugeridos nivel 2)
      const suggestedData = await apiService.getSuggested();
      const formattedSuggested = suggestedData.map((suggested, index) => {
        const username = typeof suggested === 'string' ? suggested : suggested.username;
        const compatibility = typeof suggested === 'object' && suggested.compatibility
          ? Math.round(suggested.compatibility * 100)
          : Math.floor(Math.random() * 50) + 30;
        
        return {
          id: `suggested_${index}`,
          usuario: username,
          avatar: suggested.avatar || defaultAvatar,
          compatibilidad: compatibility,
        };
      });
      setRecommended(formattedSuggested);

    } catch (error) {
      console.error('Error cargando matches:', error);
    } finally {
      setLoading(false);
    }
  }

  if (loading) {
    return (
      <div className="bg-gray-800 text-white w-full h-full flex items-center justify-center">
        <p>Cargando...</p>
      </div>
    );
  }

  return (
    <div className="bg-gray-800 text-white w-full h-full overflow-y-auto p-5">
      
      {/* Header */}
      <button 
        onClick={() => navigate("/feed")}
        className="mb-4 text-xl hover:text-gray-400"
      >
        ← Volver
      </button>

      <h2 className="text-2xl font-bold mb-6">Tus Matches</h2>

      {/* Todos mis matches */}
      <div className="mb-10">
        <div className="grid grid-cols-2 sm:grid-cols-2 md:grid-cols-2 gap-4">
          {matches.map(m => (
            <div 
              key={m.id} 
              className="flex flex-col items-center cursor-pointer hover:scale-105 transition"
              onClick={() => navigate(`/profile/${m.usuario}`)}
            >
              <div className="p-[3px] rounded-full bg-gradient-to-r from-purple-500 to-orange-500">
                <img
                  src={m.avatar}
                  className="w-24 h-24 rounded-full bg-gray-700 object-cover"
                />
              </div>
              
              <p className="mt-2 font-medium text-center">@{m.usuario}</p>
              <p className="text-xs text-gray-300">
                <span className="text-purple-300 font-bold">Compatibilidad: <span className="text-purple-300 font-bold">{m.compatibilidad}%</span></span>
              </p>
            </div>
          ))}
        </div>

        {matches.length === 0 && (
          <p className="text-gray-400 text-center">No tienes matches aún</p>
        )}
      </div>

      {/* Personas recomendadas */}
      <div className="pb-10">
        <h3 className="text-xl font-semibold mb-4">Personas recomendadas</h3>
        
        <div className="space-y-3">
          {recommended.map(r => (
            <div
              key={r.id}
              className="flex items-center bg-gray-700 p-4 rounded-xl hover:bg-gray-600 transition"
            >
              <img
                src={r.avatar}
                className="w-16 h-16 rounded-full bg-gray-600 object-cover mr-4 cursor-pointer"
                onClick={() => navigate(`/profile/${r.usuario}`)}
              />

              <div 
                className="flex-1 cursor-pointer"
                onClick={() => navigate(`/profile/${r.usuario}`)}
              >
                <p className="font-semibold text-lg">@{r.usuario}</p>
                <p className="text-sm text-gray-300">
                  Compatibilidad:{" "}
                  <span className="font-bold text-purple-300">
                    {r.compatibilidad}%
                  </span>
                </p>
              </div>

              <button 
                className="bg-indigo-500 px-4 py-2 rounded-lg text-sm font-semibold hover:bg-indigo-400 transition"
                onClick={() => navigate(`/profile/${r.usuario}`)}
              >
                Ver perfil
              </button>
            </div>
          ))}
        </div>

        {recommended.length === 0 && (
          <p className="text-gray-400 text-center">No hay recomendaciones por ahora</p>
        )}
      </div>
    </div>
  );
}