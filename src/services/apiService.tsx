import { authService } from './authService';

const API_URL = 'https://min.oxlo.io:6232';

interface MatchUser {
  username: string;
  first_name?: string;
  last_name?: string;
  description?: string;
  avatar?: string;
  compatibility?: number;
}

interface Interest {
  name: string;
  description: string;
  type: string;
  count?: number;
}

interface SearchResult {
  username: string;
  first_name?: string;
  last_name?: string;
  avatar?: string;
  compatibility?: number;
}

export const apiService = {
  // Obtener tus matches
  async getMyMatches(): Promise<MatchUser[]> {
    const token = authService.getToken();
    
    const response = await fetch(`${API_URL}/me/matches`, {
      method: 'GET',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
    });

    if (!response.ok) {
      throw new Error('Error al obtener matches');
    }

    const data = await response.json();
    // El backend puede devolver { matches: [...] } o directamente un array
    return data.matches || data || [];
  },

  // Obtener sugeridos (nivel 2)
  async getSuggested(): Promise<MatchUser[]> {
    const token = authService.getToken();
    
    const response = await fetch(`${API_URL}/me/lv2`, {
      method: 'GET',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
    });

    if (!response.ok) {
      throw new Error('Error al obtener sugeridos');
    }

    const data = await response.json();
    return data.matches || data || [];
  },

  // Dar match con alguien
  async createMatch(targetUsername: string): Promise<void> {
    const token = authService.getToken();
    
    const response = await fetch(`${API_URL}/me/match`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ target: targetUsername }),
    });

    if (!response.ok) {
      throw new Error('Error al crear match');
    }
  },

  // Quitar match con alguien
  async deleteMatch(targetUsername: string): Promise<void> {
    const token = authService.getToken();
    
    const response = await fetch(`${API_URL}/me/match`, {
      method: 'DELETE',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ target: targetUsername }),
    });

    if (!response.ok) {
      throw new Error('Error al eliminar match');
    }
  },

// Obtener matches de otro usuario
async getOtherMatches(username: string): Promise<MatchUser[]> {
  const token = authService.getToken();
  
  const response = await fetch(`${API_URL}/other/matches`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ username }),
  });

  if (!response.ok) {
    throw new Error('Error al obtener matches del usuario');
  }

  const data = await response.json();
  return data.matches || data || [];
},

  // Obtener tus intereses/gustos
  async getMyInterests(): Promise<Interest[]> {
    const token = authService.getToken();
    
    const response = await fetch(`${API_URL}/me/interest`, {
      method: 'GET',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
    });

    if (!response.ok) {
      throw new Error('Error al obtener intereses');
    }

    const data = await response.json();
    return data.interests || [];
  },

  // Dejar de seguir un interés
  async unlikeInterest(interestName: string): Promise<void> {
    const token = authService.getToken();
    
    const response = await fetch(`${API_URL}/me/interest`, {
      method: 'DELETE',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ name: interestName }),
    });

    if (!response.ok) {
      throw new Error('Error al eliminar interés');
    }
  },

// Obtener intereses de otro usuario
async getOtherInterests(username: string): Promise<Interest[]> {
  const token = authService.getToken();
  
  const response = await fetch(`${API_URL}/other/interest`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ username }),
  });

  if (!response.ok) {
    throw new Error('Error al obtener intereses del usuario');
  }

  const data = await response.json();
  return data.interests || [];
},

// Buscar usuarios
async searchUsers(term: string): Promise<SearchResult[]> {
  const token = authService.getToken();
  
  const response = await fetch(`${API_URL}/other/search`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ term }),
  });

  if (!response.ok) {
    throw new Error('Error al buscar usuarios');
  }

  const data = await response.json();
  
  return data.matches || [];
},

    // Obtener recomendaciones de intereses
// Obtener recomendaciones de intereses
async getRecommendedInterests(): Promise<Interest[]> {
  const token = authService.getToken();
  
  const response = await fetch(`${API_URL}/me/recommendations`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) {
    throw new Error('Error al obtener recomendaciones');
  }

  const data = await response.json();
  console.log('Respuesta de recomendaciones:', data);
  return data.recommendations || [];
},

    // Agregar un interés
    async likeInterest(interestName: string): Promise<void> {
    const token = authService.getToken();
    
    const response = await fetch(`${API_URL}/me/interest`, {
        method: 'POST',
        headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json',
        },
        body: JSON.stringify({ name: interestName }),
    });

    if (!response.ok) {
        throw new Error('Error al agregar interés');
    }
    },

// Obtener información de un usuario
async getUserInfo(username: string): Promise<any> {
  const token = authService.getToken();
  
  const response = await fetch(`${API_URL}/other`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ username }),
  });

  if (!response.ok) {
    throw new Error('Error al obtener info del usuario');
  }

  return await response.json();
},

// Obtener PageRank (tendencias)
async getPageRank(): Promise<{ rankings: { name: string; score: number }[] }> {
  const token = authService.getToken();
  
  const response = await fetch(`${API_URL}/pagerank`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) {
    throw new Error('Error al obtener tendencias');
  }

  return await response.json();
},
};