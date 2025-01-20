import { useRouter } from 'vue-router'

export function useAuth() {
  const router = useRouter()

  const registerUser = async (username: string, email: string, password: string) => {
    try {
      const response = await fetch('http://localhost:80/api/v1/users/register', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ name: username, email, password }),
      })

      if (!response.ok) {
        const error = await response.json()
        alert(`Erro: ${error.message}`)
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
      const response = await fetch('http://localhost:80/api/v1/users/login', {
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
      alert(`Erro ao efetuar login: ${error.message}`)
    }
  }

  const getUser = async () => {
    try {
      const response = await fetch('http://localhost:80/api/v1/users/me', {
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
      alert(`Error obtaining user: ${error.message}`);
      throw error;
    }
  };

  const changePassword = async (current_password: string, new_password: string) => {
    try {
      const response = await fetch('http://localhost:80/api/v1/users/changepassword', {
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

  return { registerUser, loginUser, getUser, changePassword }
}
