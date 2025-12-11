import { useEffect, useState, useRef } from "react";
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

interface Interest {
  name: string;
  description: string;
  type: string;
}

interface SearchResult {
  username: string;
  first_name?: string;
  last_name?: string;
  avatar?: string;
  compatibility?: number;
}

interface TrendingGenre {
  name: string;
  score: number;
}

export default function Feed() {
  const navigate = useNavigate();

  const [matches, setMatches] = useState<MatchUser[]>([]);
  const [chats, setChats] = useState<Chat[]>([]);
  const [sugeridos, setSugeridos] = useState<Sugerido[]>([]);
  const [interests, setInterests] = useState<Interest[]>([]);
  const [loading, setLoading] = useState(true);
  const [showModal, setShowModal] = useState(false);
  const [selectedInterest, setSelectedInterest] = useState<Interest | null>(null);
  
  // Estados para búsqueda
  const [searchTerm, setSearchTerm] = useState("");
  const [searchResults, setSearchResults] = useState<SearchResult[]>([]);
  const [showSearchDropdown, setShowSearchDropdown] = useState(false);
  const [searchLoading, setSearchLoading] = useState(false);
  const searchRef = useRef<HTMLDivElement>(null);

  // Estados para tendencias
  const [trending, setTrending] = useState<TrendingGenre[]>([]);
  const [trendingLoading, setTrendingLoading] = useState(false);

  const loadChats = () => {
    const savedChats = localStorage.getItem('chatsRecientes');
    if (savedChats) {
      setChats(JSON.parse(savedChats));
    } else {
      const defaultChats = [
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
      ];
      setChats(defaultChats);
      localStorage.setItem('chatsRecientes', JSON.stringify(defaultChats));
    }
  };

  const loadData = async () => {
    try {
      setLoading(true);

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
      localStorage.setItem('myMatches', JSON.stringify(formattedMatches));

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
      setSugeridos(formattedSuggested);

      const interestsData = await apiService.getMyInterests();
      setInterests(interestsData);
      localStorage.setItem('myInterests', JSON.stringify(interestsData)); 

      loadChats();

    } catch (error) {
      console.error('Error cargando datos:', error);
      
      const savedMatches = localStorage.getItem('myMatches');
      if (savedMatches) {
        setMatches(JSON.parse(savedMatches));
      }
      
      loadChats();
    } finally {
      setLoading(false);
    }
  };

  const loadTrending = async () => {
    try {
      setTrendingLoading(true);
      const data = await apiService.getPageRank();
      setTrending(data.rankings.slice(0, 10)); // Top 10
    } catch (error) {
      console.error('Error cargando tendencias:', error);
    } finally {
      setTrendingLoading(false);
    }
  };

  useEffect(() => {
    loadData();
    loadTrending();
  }, []);

  useEffect(() => {
    const handleFocus = () => {
      loadChats();
    };

    window.addEventListener('focus', handleFocus);
    
    return () => {
      window.removeEventListener('focus', handleFocus);
    };
  }, []);

  useEffect(() => {
    const interval = setInterval(() => {
      loadChats();
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  // Click fuera del dropdown para cerrarlo
  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (searchRef.current && !searchRef.current.contains(event.target as Node)) {
        setShowSearchDropdown(false);
      }
    }

    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  // Búsqueda con debounce
  useEffect(() => {
    if (searchTerm.length < 2) {
      setSearchResults([]);
      setShowSearchDropdown(false);
      return;
    }

const timeoutId = setTimeout(async () => {
    try {
      setSearchLoading(true);
      const results = await apiService.searchUsers(searchTerm);
      console.log('Resultados de búsqueda:', results);
      
      // 👇 ARREGLAR: Extraer el array de matches
      const matchesArray = results.matches || results || [];
      setSearchResults(matchesArray); // 👈 Usar matchesArray en lugar de results
      setShowSearchDropdown(true);
    } catch (error) {
      console.error('Error buscando usuarios:', error);
      setSearchResults([]);
    } finally {
      setSearchLoading(false);
    }
  }, 500);


    return () => clearTimeout(timeoutId);
  }, [searchTerm]);

  function handleUnlikeInterest(interest: Interest) {
    setSelectedInterest(interest);
    setShowModal(true);
  }

  async function confirmUnlike() {
    if (!selectedInterest) return;

    try {
      await apiService.unlikeInterest(selectedInterest.name);
      
      setInterests(interests.filter(i => i.name !== selectedInterest.name));
      
      setShowModal(false);
      setSelectedInterest(null);
    } catch (error) {
      console.error('Error al eliminar interés:', error);
      alert('Error al quitar este interés. Intenta de nuevo.');
      setShowModal(false);
    }
  }

  function cancelUnlike() {
    setShowModal(false);
    setSelectedInterest(null);
  }

  function handleSelectUser(username: string) {
    setSearchTerm("");
    setShowSearchDropdown(false);
    navigate(`/profile/${username}`);
  }

  if (loading) {
    return (
      <div className="bg-gray-800 text-white w-full h-full flex items-center justify-center">
        <p>Cargando...</p>
      </div>
    );
  }

  return (
    <div className="bg-gray-800 text-white w-full h-full overflow-y-auto">
      
      {/* BARRA DE BÚSQUEDA */}
      <div className="sticky top-0 bg-gray-800 p-5 pb-3 border-b border-gray-700 z-10">
        <div className="relative" ref={searchRef}>
          <input
            type="text"
            placeholder="🔍 Buscar usuarios..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="w-full bg-gray-700 text-white px-4 py-3 rounded-xl 
            focus:outline-none focus:ring-2 focus:ring-purple-500 transition"
          />

        {/* Dropdown de resultados */}
        {showSearchDropdown && (
          <div className="absolute top-full left-0 right-0 mt-2 bg-gray-900 rounded-xl 
          border border-gray-700 shadow-2xl max-h-96 overflow-y-auto z-20">
            {searchLoading ? (
              <div className="p-4 text-center text-gray-400">
                Buscando...
              </div>
            ) : searchResults.length > 0 ? (
              searchResults.map((user, idx) => (
                <div
                  key={idx}
                  onClick={() => handleSelectUser(user.username)}
                  className="flex items-center p-3 hover:bg-gray-700 cursor-pointer 
                  transition border-b border-gray-800 last:border-b-0"
                >
                  <img
                    src={user.avatar || defaultAvatar}
                    className="w-12 h-12 rounded-full bg-gray-700 object-cover mr-3"
                  />
                  <div className="flex-1 min-w-0">
                    <p className="font-semibold truncate">@{user.username}</p>
                    {(user.first_name || user.last_name) && (
                      <p className="text-sm text-gray-400 truncate">
                        {user.first_name} {user.last_name}
                      </p>
                    )}
                  </div>
                  {user.compatibility !== undefined && user.compatibility > 0 && (
                    <span className="text-sm text-purple-300 font-semibold ml-2 shrink-0">
                      {Math.round(user.compatibility * 100)}%
                    </span>
                  )}
                </div>
              ))
            ) : (
              <div className="p-6 text-center">
                <p className="text-gray-400 mb-1">Sin resultados</p>
                <p className="text-sm text-gray-500">No encontramos usuarios con "{searchTerm}"</p>
              </div>
            )}
          </div>
        )}
        </div>
      </div>

      <div className="p-5 pt-3">
        {/* MATCHES */}
        <div>
          <div className="flex items-center justify-between mb-2">
            <h3 className="text-lg font-semibold">Tus Matches</h3>
            {matches.length > 2 && (
              <button
                onClick={() => navigate('/matches')}
                className="text-sm text-purple-400 hover:text-purple-300 flex items-center gap-1"
              >
                Ver más →
              </button>
            )}
          </div>

          <div className="flex space-x-4 overflow-x-auto pb-3">
            {matches.slice(0, 2).map(m => (
              <div 
                key={m.id} 
                className="flex flex-col items-center cursor-pointer"
                onClick={() => navigate(`/profile/${m.usuario}`)}
              >
                <div className="p-[3px] rounded-full bg-gradient-to-r from-purple-500 to-orange-500">
                  <img
                    src={m.avatar}
                    className="w-20 h-20 rounded-full bg-gray-700 object-cover"
                  />
                </div>
                
                <p className="mt-2 font-medium">@{m.usuario}</p>
                <p className="text-xs text-gray-300">
                  Compatibilidad:{" "}
                  <span className="text-purple-300 font-bold">{m.compatibilidad}%</span>
                </p>
              </div>
            ))}
          </div>

          {matches.length === 0 && (
            <p className="text-gray-400 text-sm">No tienes matches aún</p>
          )}
        </div>

        {/* MIS INTERESES */}
        <div className="mt-6">
          <div className="flex items-center justify-between mb-2">
            <h3 className="text-lg font-semibold">🎯 Mis intereses</h3>
            {interests.length > 5 && (
              <button
                onClick={() => navigate('/interests')}
                className="text-sm text-purple-400 hover:text-purple-300 flex items-center gap-1"
              >
                Ver más →
              </button>
            )}
          </div>

          <div className="flex flex-wrap gap-2">
            {interests.slice(0, 5).map((interest, idx) => (
              <div
                key={idx}
                onClick={() => handleUnlikeInterest(interest)}
                className="bg-gradient-to-r from-purple-500 to-orange-500 px-4 py-2 rounded-full 
                text-sm font-medium cursor-pointer hover:opacity-80 transition flex items-center gap-2"
              >
                <span>{interest.name}</span>
                <span className="text-xs opacity-70">✕</span>
              </div>
            ))}
          </div>

          {interests.length === 0 && (
            <p className="text-gray-400 text-sm">No tienes intereses agregados aún</p>
          )}
        </div>

        {/* CHATS */}
        <div className="mt-6">
          <h3 className="text-lg font-semibold mb-2">💬 Chats recientes</h3>

          <div className="space-y-3">
            {chats.map(c => (
              <div
                key={c.id}
                className="flex items-center bg-gray-700 p-3 rounded-xl cursor-pointer hover:bg-gray-600"
                onClick={() => navigate(`/chat/${c.id}`, { state: { usuario: c.usuario, avatar: c.avatar } })}
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
        <div className="mt-8">
          <h3 className="text-lg font-semibold mb-2">Personas sugeridas</h3>

          <div className="space-y-4">
            {sugeridos.map(s => (
              <div
                key={s.id}
                className="flex items-center bg-gray-700 p-3 rounded-xl hover:bg-gray-600"
              >
                <img
                  src={s.avatar || defaultAvatar} 
                  className="w-16 h-16 rounded-full bg-gray-700 object-cover mr-3 cursor-pointer"
                  onClick={() => navigate(`/profile/${s.usuario}`)}
                />

                <div 
                  className="flex-1 cursor-pointer"
                  onClick={() => navigate(`/profile/${s.usuario}`)}
                >
                  <p className="font-semibold">@{s.usuario}</p>
                  <p className="text-sm text-gray-300">
                    Compatibilidad:{" "}
                    <span className="font-bold text-purple-300">
                      {s.compatibilidad}%
                    </span>
                  </p>
                </div>

                <button 
                  className="bg-indigo-500 px-3 py-1.5 rounded-md text-sm hover:bg-indigo-400"
                  onClick={() => navigate(`/profile/${s.usuario}`)}
                >
                  Ver perfil
                </button>
              </div>
            ))}
          </div>
        </div>

        {/* TENDENCIAS */}
        <div className="mt-8 pb-10">
          <h3 className="text-lg font-semibold mb-3">Tendencias</h3>

          {trendingLoading ? (
            <p className="text-gray-400 text-sm">Cargando tendencias...</p>
          ) : trending.length > 0 ? (
            <div className="space-y-2">
              {trending.map((genre, idx) => (
                <div
                  key={idx}
                  className="flex items-center justify-between bg-gray-700 p-3 rounded-xl hover:bg-gray-600 transition"
                >
                  <div className="flex items-center gap-3">
                    <span className="text-2xl font-bold text-gray-500">#{idx + 1}</span>
                    <div>
                      <p className="font-semibold">{genre.name}</p>
                      <p className="text-xs text-gray-400">Puntuación: {genre.score.toFixed(4)}</p>
                    </div>
                  </div>
                  <span className="text-2xl">🔥</span>
                </div>
              ))}
            </div>
          ) : (
            <p className="text-gray-400 text-sm">No hay tendencias disponibles</p>
          )}
        </div>
      </div>

      {/* Modal de confirmación */}
      {showModal && selectedInterest && (
        <div className="fixed inset-0 bg-black bg-opacity-70 flex items-center justify-center z-50 p-4">
          <div className="bg-gray-900 rounded-2xl p-6 max-w-sm w-full border border-gray-700 shadow-2xl">
            <h3 className="text-xl font-bold mb-3 text-center">¿Ya no te gusta?</h3>
            <p className="text-gray-300 text-center mb-6">
              ¿Estás segura(o) que <span className="font-semibold text-purple-300">{selectedInterest.name}</span> ya no te interesa?
            </p>
            
            <div className="flex space-x-3">
              <button
                onClick={cancelUnlike}
                className="flex-1 bg-gray-700 py-3 rounded-xl font-semibold 
                hover:bg-gray-600 transition"
              >
                Cancelar
              </button>
              <button
                onClick={confirmUnlike}
                className="flex-1 bg-red-500 py-3 rounded-xl font-semibold 
                hover:bg-red-400 transition"
              >
                Sí, quitar
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}