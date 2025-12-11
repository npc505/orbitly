import { useState, useEffect } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { apiService } from "../services/apiService";

const defaultAvatar =
  "https://upload.wikimedia.org/wikipedia/commons/thumb/5/59/User-avatar.svg/2048px-User-avatar.svg.png";

interface Interest {
  name: string;
  description?: string;
  type?: string;
}

export default function Profile() {
  const navigate = useNavigate();
  const { userId } = useParams();
  const username = userId || "";
  
  const [interests, setInterests] = useState<Interest[]>([]);
  const [matches, setMatches] = useState<string[]>([]);
  const [isMatch, setIsMatch] = useState(false);
  const [showModal, setShowModal] = useState(false);
  const [loading, setLoading] = useState(true);
  const [compatibility, setCompatibility] = useState<number>(0);
  const [canViewMatches, setCanViewMatches] = useState(false);
  const [commonInterestsCount, setCommonInterestsCount] = useState(0);
  const [userAvatar, setUserAvatar] = useState<string>(defaultAvatar); 

  useEffect(() => {
    loadProfile();
  }, [username]);

async function loadProfile() {
  if (!username) return;

  try {
    setLoading(true);

    // Obtener info del usuario (incluyendo avatar)
    try {
      const userInfo = await apiService.getUserInfo(username);
      setUserAvatar(userInfo.avatar || defaultAvatar);
    } catch (error) {
      console.error('Error obteniendo info del usuario:', error);
      setUserAvatar(defaultAvatar);
    }

    // Cargar intereses del usuario
    const userInterests = await apiService.getOtherInterests(username);
    setInterests(userInterests);

    // Verificar si ya es match PRIMERO
    const myMatches = await apiService.getMyMatches();
    const alreadyMatch = myMatches.some(
      (m) => (typeof m === 'string' ? m : m.username) === username
    );
    setIsMatch(alreadyMatch);
    setCanViewMatches(alreadyMatch);

    // Solo cargar matches si ya son match
    if (alreadyMatch) {
      try {
        const userMatchesData = await apiService.getOtherMatches(username);
        const userMatchesUsernames = userMatchesData.map((match) => 
          typeof match === 'string' ? match : match.username
        );
        setMatches(userMatchesUsernames);
      } catch (error) {
        console.error('Error cargando matches:', error);
        setMatches([]);
      }
    } else {
      setMatches([]);
    }

    // Calcular compatibilidad
    try {
      const myInterests = await apiService.getMyInterests();
      const commonInterests = userInterests.filter(ui => 
        myInterests.some(mi => mi.name === ui.name)
      );
      
      setCommonInterestsCount(commonInterests.length);
      
      // Compatibilidad basada en sus intereses
      const totalInterests = Math.max(userInterests.length, 1);
      const compatibilityScore = Math.round((commonInterests.length / totalInterests) * 100);
      setCompatibility(Math.min(compatibilityScore, 100));
    } catch (error) {
      console.error('Error calculando compatibilidad:', error);
      setCompatibility(0);
      setCommonInterestsCount(0);
    }

  } catch (error) {
    console.error('Error cargando perfil:', error);
  } finally {
    setLoading(false);
  }
}
  async function handleConnect() {
    try {
      await apiService.createMatch(username);
      setIsMatch(true);
      setCanViewMatches(true);
      
      // Actualizar localStorage
      const myMatches = await apiService.getMyMatches();
      localStorage.setItem('myMatches', JSON.stringify(myMatches));
      
      // Cargar matches ahora que son match
      const userMatchesData = await apiService.getOtherMatches(username);
      const userMatchesUsernames = userMatchesData.map((match) => 
        typeof match === 'string' ? match : match.username
      );
      setMatches(userMatchesUsernames);
    } catch (error) {
      console.error('Error al crear match:', error);
      alert('Error al conectar. Intenta de nuevo.');
    }
  }

  function handleDisconnect() {
    setShowModal(true);
  }

  async function confirmDisconnect() {
    try {
      await apiService.deleteMatch(username);
      setIsMatch(false);
      setCanViewMatches(false);
      setMatches([]);
      setShowModal(false);
      
      // Actualizar localStorage
      const myMatches = await apiService.getMyMatches();
      localStorage.setItem('myMatches', JSON.stringify(myMatches));
      
      setTimeout(() => {
        navigate("/feed");
      }, 500);
    } catch (error) {
      console.error('Error al eliminar match:', error);
      alert('Error al dejar de seguir. Intenta de nuevo.');
      setShowModal(false);
    }
  }

  function cancelDisconnect() {
    setShowModal(false);
  }

  if (loading) {
    return (
      <div className="bg-gray-800 text-white w-full h-full flex items-center justify-center">
        <p>Cargando perfil...</p>
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

{/* Foto de perfil */}
<div className="flex flex-col items-center mb-6">
  <div className="p-[3px] rounded-full bg-gradient-to-r from-purple-500 to-orange-500">
    <img
      src={userAvatar}
      className="w-32 h-32 rounded-full bg-gray-700 object-cover"
    />
  </div>
  <h2 className="mt-4 text-2xl font-bold">{username}</h2>
  <p className="text-gray-400">@{username}</p>
</div>

      {/* Compatibilidad */}
      <div className="mb-6 bg-gray-700 p-4 rounded-xl">
        {/* Botones */}
        {!isMatch ? (
          <button
            onClick={handleConnect}
            className="mt-4 w-full bg-indigo-500 py-3 rounded-xl font-semibold 
            hover:bg-indigo-400 transition"
          >
            Conectar
          </button>
        ) : (
          <button
            onClick={handleDisconnect}
            className="mt-4 w-full bg-red-500 py-3 rounded-xl font-semibold 
            hover:bg-red-400 transition"
          >
            Dejar de seguir
          </button>
        )}
      </div>

      {/* Gustos */}
      <div className="mb-6">
        <h3 className="text-lg font-semibold mb-3">Intereses ({interests.length})</h3>
        <div className="flex flex-wrap gap-2">
          {interests.map((interest, idx) => (
            <div key={idx} className="bg-indigo-500 px-4 py-2 rounded-full">
              <p className="text-sm font-medium">{interest.name}</p>
              {interest.type && (
                <p className="text-xs opacity-80 capitalize">{interest.type}</p>
              )}
            </div>
          ))}
        </div>
        {interests.length === 0 && (
          <p className="text-gray-400">No tiene intereses públicos</p>
        )}
      </div>

      {/* Matches del usuario */}
      <div className="pb-10">
        <h3 className="text-lg font-semibold mb-3">
          Sus matches {canViewMatches ? `(${matches.length})` : '🔒'}
        </h3>
        
        {canViewMatches ? (
          <>
            {matches.length > 0 ? (
              <div className="flex space-x-4 overflow-x-auto pb-2">
                {matches.map((match, idx) => (
                  <div 
                    key={idx} 
                    className="flex flex-col items-center cursor-pointer hover:scale-105 transition"
                    onClick={() => navigate(`/profile/${match}`)}
                  >
                    <div className="p-[2px] rounded-full bg-gradient-to-r from-purple-500 to-orange-500">
                      <img
                        src={defaultAvatar}
                        className="w-16 h-16 rounded-full bg-gray-700 object-cover"
                      />
                    </div>
                    <p className="mt-2 text-sm">@{match}</p>
                  </div>
                ))}
              </div>
            ) : (
              <p className="text-gray-400">No tiene matches públicos</p>
            )}
          </>
        ) : (
          <div className="bg-gray-700 p-6 rounded-xl text-center">
            <p className="text-gray-300 mb-2">🔒 No tienes permiso para ver sus matches</p>
            <p className="text-sm text-gray-400">Conecta con @{username} para ver sus matches</p>
          </div>
        )}
      </div>

      {/* Modal */}
      {showModal && (
        <div className="fixed inset-0 bg-black bg-opacity-70 flex items-center justify-center z-50 p-4">
          <div className="bg-gray-900 rounded-2xl p-6 max-w-sm w-full border border-gray-700 shadow-2xl">
            <h3 className="text-xl font-bold mb-3 text-center">¿Dejar de seguir?</h3>
            <p className="text-gray-300 text-center mb-6">
              ¿Estás segura(o) que quieres dejar de seguir a <span className="font-semibold text-purple-300">@{username}</span>?
            </p>
            
            <div className="flex space-x-3">
              <button
                onClick={cancelDisconnect}
                className="flex-1 bg-gray-700 py-3 rounded-xl font-semibold 
                hover:bg-gray-600 transition"
              >
                Cancelar
              </button>
              <button
                onClick={confirmDisconnect}
                className="flex-1 bg-red-500 py-3 rounded-xl font-semibold 
                hover:bg-red-400 transition"
              >
                Dejar de seguir
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}