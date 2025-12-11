import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { apiService } from "../services/apiService";

interface Interest {
  name: string;
  description?: string;
  type?: string;
  count?: number;
}

export default function Interests() {
  const navigate = useNavigate();
  const [myInterests, setMyInterests] = useState<Interest[]>([]);
  const [seenInterests, setSeenInterests] = useState<string[]>([]);
  const [allRecommended, setAllRecommended] = useState<Interest[]>([]);
  const [displayedInterests, setDisplayedInterests] = useState<Interest[]>([]);
  const [loading, setLoading] = useState(true);
  const [refreshing, setRefreshing] = useState(false);
  const [showModal, setShowModal] = useState(false);
  const [selectedInterest, setSelectedInterest] = useState<Interest | null>(null);

useEffect(() => {
  loadInterests();
}, []);


async function loadInterests() {
  try {
    setLoading(true);

    // Cargar mis intereses
    const myData = await apiService.getMyInterests();
    setMyInterests(myData);

    // Cargar intereses recomendados desde el backend
    const recommendedData = await apiService.getRecommendedInterests();
    setAllRecommended(recommendedData);

    // Guardar en historial (los ya vistos)
    setSeenInterests(recommendedData.map(i => i.name));

    // Mostrar solo los primeros 4
    setDisplayedInterests(recommendedData.slice(0, 4));

  } catch (error) {
    console.error('Error cargando intereses:', error);
  } finally {
    setLoading(false);
  }
}


  function handleUnlikeInterest(interest: Interest) {
    setSelectedInterest(interest);
    setShowModal(true);
  }

  async function confirmUnlike() {
    if (!selectedInterest) return;

    try {
      await apiService.unlikeInterest(selectedInterest.name);
      
      setMyInterests(myInterests.filter(i => i.name !== selectedInterest.name));
      
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

async function handleAddInterest(interest: Interest) {
  try {
    await apiService.likeInterest(interest.name);
    
    // Agregar a mis intereses
    const updatedMyInterests = [...myInterests, interest];
    setMyInterests(updatedMyInterests);
    
    // Quitar de los mostrados
    const newDisplayed = displayedInterests.filter(i => i.name !== interest.name);
    
    // Buscar el siguiente interés que no esté en mis intereses ni en los mostrados
    const usedNames = [...updatedMyInterests.map(i => i.name), ...newDisplayed.map(i => i.name)]; // 👈 Usar updatedMyInterests
    const nextInterest = allRecommended.find(
      rec => !usedNames.includes(rec.name)
    );
    
    // Si hay un siguiente, agregarlo; si no, mantener los que quedan
    if (nextInterest) {
      setDisplayedInterests([...newDisplayed, nextInterest]);
    } else {
      setDisplayedInterests(newDisplayed);
    }
    
    console.log('Interés agregado:', interest.name); // 👈 AGREGAR
    console.log('Mostrados ahora:', newDisplayed.length + (nextInterest ? 1 : 0)); // 👈 AGREGAR
    
  } catch (error) {
    console.error('Error al agregar interés:', error);
    alert('Error al agregar este interés. Intenta de nuevo.');
  }
}

async function handleRefresh() {
  try {
    setRefreshing(true);
    
    // Recargar recomendaciones desde el backend
    const recommendedData = await apiService.getRecommendedInterests();
    console.log('Recomendaciones recibidas:', recommendedData.length);
    
    setAllRecommended(recommendedData);
    
    // Filtrar los que ya tengo
    const filtered = recommendedData.filter(
      rec => !myInterests.some(my => my.name === rec.name)
    );
    
    console.log('Después de filtrar:', filtered.length);
    
    if (filtered.length === 0) {
      alert('¡Ya tienes todos los intereses recomendados! 🎉');
      setDisplayedInterests([]);
    } else {
      // Mostrar 4 nuevos (o los que queden si son menos de 4)
      const toShow = filtered.slice(0, 4);
      console.log('Mostrando:', toShow.map(i => i.name)); // 👈 AGREGAR
      setDisplayedInterests(toShow);
    }
    
  } catch (error) {
    console.error('Error al refrescar:', error);
    alert('Error al buscar más intereses. Intenta de nuevo.');
  } finally {
    setRefreshing(false);
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

      <h2 className="text-2xl font-bold mb-6">Mis Intereses</h2>

      {/* Mis intereses */}
      <div className="mb-8">
        <h3 className="text-lg font-semibold mb-3">Tus intereses actuales</h3>
        <div className="flex flex-wrap gap-3">
        {myInterests.map((interest, idx) => (
            <div
            key={idx}
            onClick={() => handleUnlikeInterest(interest)}
            className="bg-gradient-to-r from-purple-500 to-orange-500 px-5 py-3 rounded-xl 
            text-base font-medium cursor-pointer hover:opacity-80 transition flex items-center gap-2"
            >
            <div>
                <p className="font-semibold">{interest.name}</p>
                {interest.type && (
                <p className="text-xs opacity-80 capitalize">{interest.type}</p>
                )}
            </div>
            <span className="text-sm opacity-70 ml-2">✕</span>
            </div>
        ))}
        </div>

        {myInterests.length === 0 && (
          <p className="text-gray-400">No tienes intereses agregados aún</p>
        )}
      </div>

{/* Explorar más */}
<div className="pb-10">
  <h3 className="text-lg font-semibold mb-3">Explorar más intereses</h3>
  
  {displayedInterests.length > 0 ? (
    <>
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-3 mb-4">
        {displayedInterests.map((interest, idx) => (
          <div
            key={idx}
            className="bg-gray-700 p-4 rounded-xl hover:bg-gray-600 transition"
          >
            <p className="font-semibold text-lg mb-3">{interest.name}</p>
            {interest.type && (
              <p className="text-sm text-gray-400 mb-3 capitalize">{interest.type}</p>
            )}
            <button 
              onClick={() => handleAddInterest(interest)}
              className="bg-indigo-500 px-4 py-2 rounded-lg text-sm hover:bg-indigo-400 transition w-full"
            >
              + Agregar interés
            </button>
          </div>
        ))}
      </div>

      {/* Botón de refresh */}
      <button
        onClick={handleRefresh}
        disabled={refreshing}
        className="w-full bg-gray-700 hover:bg-gray-600 px-4 py-3 rounded-xl 
        font-semibold transition flex items-center justify-center gap-2 disabled:opacity-50"
      >
        {refreshing ? (
          <>
            <span className="animate-spin">🔄</span>
            Buscando...
          </>
        ) : (
          <>
            🔄 Buscar más intereses
          </>
        )}
      </button>
    </>
  ) : (
    <div className="text-center py-8">
      <p className="text-gray-400 mb-4">No hay más recomendaciones por ahora</p>
      <button
        onClick={handleRefresh}
        disabled={refreshing}
        className="bg-indigo-500 hover:bg-indigo-400 px-6 py-3 rounded-xl 
        font-semibold transition disabled:opacity-50"
      >
        {refreshing ? 'Buscando...' : '🔄 Buscar más'}
      </button>
    </div>
  )}
</div>

      {/* Modal */}
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
                className="flex-1 bg-gray-700 py-3 rounded-xl font-semibold hover:bg-gray-600 transition"
              >
                Cancelar
              </button>
              <button
                onClick={confirmUnlike}
                className="flex-1 bg-red-500 py-3 rounded-xl font-semibold hover:bg-red-400 transition"
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