// src/services/authService.tsx
const API_URL = 'https://min.oxlo.io:6232';

interface LoginResponse {
  token: string;
}

interface RegisterData {
  mail: string;
  username: string;
  password: string;
  password2: string;
  first_name?: string;
  last_name?: string;
}

interface AuthError {
  message: string;
  status: number;
}

export const authService = {
  async login(username: string, password: string): Promise<LoginResponse> {
    try {
      const response = await fetch(`${API_URL}/auth/signin`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ username, password }),
      });

      if (response.status === 400) {
        throw { message: 'Completa todos los campos', status: 400 } as AuthError;
      }
      
      if (response.status === 401) {
        throw { message: 'Usuario o contraseña incorrectos', status: 401 } as AuthError;
      }
      
      if (response.status === 500) {
        throw { message: 'Error del servidor. Intenta más tarde', status: 500 } as AuthError;
      }

      if (!response.ok) {
        throw { message: 'Error al iniciar sesión', status: response.status } as AuthError;
      }

      const data: LoginResponse = await response.json();
      
      if (data.token) {
        window.sessionStorage.setItem('authToken', data.token);
      }
      
      return data;
    } catch (error: unknown) {
      if (error && typeof error === 'object' && 'status' in error) {
        throw error;
      }
      throw { message: 'Error de conexión. Verifica tu internet', status: 0 } as AuthError;
    }
  },

  async register(data: RegisterData): Promise<void> {
    try {
      const response = await fetch(`${API_URL}/auth/signup`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
      });

      if (response.status === 400) {
        throw { message: 'Verifica que todos los campos estén completos y las contraseñas coincidan', status: 400 } as AuthError;
      }

      if (response.status === 409) {
        throw { message: 'El usuario o correo ya está registrado', status: 409 } as AuthError;
      }
      
      if (response.status === 500) {
        throw { message: 'Error del servidor. Intenta más tarde', status: 500 } as AuthError;
      }

      if (!response.ok) {
        throw { message: 'Error al registrar usuario', status: response.status } as AuthError;
      }
    } catch (error: unknown) {
      if (error && typeof error === 'object' && 'status' in error) {
        throw error;
      }
      throw { message: 'Error de conexión. Verifica tu internet', status: 0 } as AuthError;
    }
  },

  logout(): void {
    window.sessionStorage.removeItem('authToken');
    window.sessionStorage.removeItem('orbitlyUser');
    
    // Limpiar todos los chats guardados
    Object.keys(localStorage).forEach(key => {
      if (key.startsWith('chat_') || key === 'chatsRecientes' || key === 'myMatches') {
        localStorage.removeItem(key);
      }
    });
  },

  getToken(): string | null {
    return window.sessionStorage.getItem('authToken');
  },

  isAuthenticated(): boolean {
    const token = window.sessionStorage.getItem('authToken');
    return !!token;
  }
};