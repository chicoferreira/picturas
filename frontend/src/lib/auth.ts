import { useRouter } from 'vue-router'
import { useUserStore } from '@/stores/user'

export function useAuth() {
  const API_BASE = import.meta.env.API_BASE || 'http://localhost:80/api/v1/';
  const router = useRouter()

  const registerUser = async (username: string, email: string, password: string) => {
    try {
      const response = await authFetch(API_BASE + 'users/register', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ name: username, email, password }),
      })

      if (!response.ok) {
        return
      }

      const data = await response.json();

      document.cookie = `access_token=${data.access_token}; Path=/`;
      document.cookie = `refresh_token=${data.refresh_token}; Path=/`;

      alert('Register successful!');
      router.push('/projects')
      
      return data;
    } catch (error: any) {
      console.error('Erro ao registar:', error.message)
      alert(`Erro ao registar: ${error.message}`)
    }
  }

  const loginUser = async (email: string, password: string) => {
    try {
      const response = await authFetch(API_BASE + 'users/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ email, password }),
      })

      if (!response.ok) {
        const error = await response.json()
        throw new Error(error.message || 'Failed to login')
      }

      const data = await response.json();

      document.cookie = `access_token=${data.access_token}; Path=/`;
      document.cookie = `refresh_token=${data.refresh_token}; Path=/`;

      alert('Login successful!');
      router.push('/projects')
      
      return data;
    } catch (error: any) {
      console.error('Erro ao efetuar login:', error.message)
    }
  }

  const logoutUser = async () => {
    try {
      const response = await authFetch(API_BASE + 'users/logout', {
        method: 'POST',
        credentials: 'include',
      });

      if (!response.ok) {
        const error = await response.json();
        throw new Error(error.message || 'Failed to logout');
      }

      const data = await response.json();

      document.cookie = `access_token=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT`;
      document.cookie = `refresh_token=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT`;

      alert('Logout successful!');
      router.push('/login')

      return data;
    } catch (error: any) {
      console.error('Error logging out:', error.message);
      alert(`Error logging out: ${error.message}`);
      throw error;
    }
  }

  const getUser = async () => {
    try {
      const response = await authFetch(API_BASE + 'users/me', {
        method: 'GET',
        credentials: 'include',
      });
  
      if (!response.ok) {
        const error = await response.json();
        throw new Error(error.message || 'Failed to get user');
      }
  
      return await response.json();
    } catch (error: any) {
      console.error('Error obtaining user:', error.message);
      return null;
    }
  };

  const changePassword = async (current_password: string, new_password: string) => {
    try {
      const response = await authFetch(API_BASE + 'users/changepassword', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify({ current_password, new_password }),
      });

      if (!response.ok) {
        const error = await response.json();
        throw new Error(error.message || 'Failed to change password');
      }

      const data = await response.json();

      document.cookie = `access_token=${data.access_token}; Path=/`;
      document.cookie = `refresh_token=${data.refresh_token}; Path=/`;

      alert('Password changed successfully!');

      return data;
    } catch (error: any) {
      console.error('Error changing password:', error.message);
      alert(`Error changing password: ${error.message}`);
      throw error;
    }
  }

  async function authFetch(input: RequestInfo | URL, init?: RequestInit) {
    const router = useRouter()
    const userStore = useUserStore()
  
    const requestOptions: RequestInit = {
      ...init,
    }
  
    // First attempt
    let response = await fetch(input, requestOptions)
  
    // If 401 => try to refresh token
    if (response.status === 401) {
      console.warn('Got 401, attempting refresh...')
  
      const refreshResponse = await fetch(API_BASE + 'users/refresh', {
        method: 'POST',
        credentials: 'include',
      })
  
      if (!refreshResponse.ok) {
        console.error('Refresh token failed, logging out user.')
        userStore.logout()
        router.push('/')
        throw new Error('Unauthorized: Refresh failed')
      }
  
      const refreshData = await refreshResponse.json()
      console.log('Tokens refreshed successfully.', refreshData)
      document.cookie = `access_token=${refreshData.access_token}; Path=/`;
      response = await fetch(input, requestOptions)
    }
  
    return response
  }

  return { registerUser, loginUser, getUser, changePassword, authFetch, logoutUser}
}
