<template>
  <div :class="themeClass" class="page-container">
    <!-- Barra lateral -->
    <div class="sidebar">
      <ul>
        <li @click="activeTab = 'profile'" :class="{ active: activeTab === 'profile' }">Perfil</li>
        <li @click="activeTab = 'account'" :class="{ active: activeTab === 'account' }">Conta</li>
        <li @click="activeTab = 'appearance'" :class="{ active: activeTab === 'appearance' }">
          Aparência
        </li>
      </ul>
      <button @click="goToHome" class="home-button">Página Inicial</button>
    </div>

    <!-- Conteúdo -->
    <div class="content">
      <div v-if="activeTab === 'profile'" class="form-container-profile">
        <h2 class="form-title">Editar Perfil</h2>
        <form @submit.prevent="updateProfile">
          <div class="form-field">
            <label for="username">Username</label>
            <input
              type="text"
              v-model="profileData.username"
              placeholder="Altere aqui o seu username"
              required
            />
          </div>
          <div class="form-field">
            <label for="email">Email</label>
            <input
              type="email"
              v-model="profileData.email"
              placeholder="Altere aqui o seu email"
              required
            />
          </div>
          <div class="form-field">
            <label for="bio">Bio</label>
            <textarea
              v-model="profileData.bio"
              placeholder="Altere aqui a sua biografia"
              required
            ></textarea>
          </div>
          <div class="buttons">
            <button type="submit" class="submit-button">Atualizar Perfil</button>
          </div>
        </form>
      </div>

      <div v-if="activeTab === 'account'" class="form-container-account">
        <h2 class="form-title">Editar Conta</h2>
        <form @submit.prevent="updateAccount">
          <div class="form-field">
            <label for="username">Username</label>
            <input
              type="text"
              v-model="accountData.username"
              placeholder="Altere aqui o seu username"
              required
            />
          </div>
          <div class="form-field">
            <label for="email">Email</label>
            <input
              type="email"
              v-model="accountData.email"
              placeholder="Altere aqui o seu email"
              required
            />
          </div>
          <div class="form-field">
            <label for="password">Senha</label>
            <input
              type="password"
              v-model="accountData.password"
              placeholder="Altere aqui a sua senha"
              required
            />
          </div>
          <div class="buttons">
            <button type="submit" class="submit-button">Atualizar Conta</button>
          </div>
        </form>
      </div>

      <div v-if="activeTab === 'appearance'" class="form-container-theme">
        <h2 class="form-title">Aparência</h2>
        <div class="switch-container">
          <label for="theme-switch" class="switch-label">Alternar Tema</label>
          <input id="theme-switch" type="checkbox" v-model="theme" />
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      activeTab: 'profile',
      theme: false,
      profileData: {
        username: 'shadcn',
        email: 'shadcn@example.com',
        bio: 'Aqui está minha biografia.',
      },
      accountData: {
        username: 'shadcn',
        email: 'shadcn@example.com',
        password: '',
      },
    }
  },
  computed: {
    themeClass() {
      return this.theme ? 'dark-theme' : 'light-theme'
    },
  },
  methods: {
    updateProfile() {
      console.log('Perfil Atualizado:', this.profileData)
      alert('Perfil atualizado com sucesso!')
    },
    updateAccount() {
      console.log('Conta Atualizada:', this.accountData)
      alert('Conta atualizada com sucesso!')
    },
    toggleTheme() {
      this.theme = !this.theme
    },
    goToHome() {
      this.$router.push('/')
    },
  },
}
</script>

<style scoped>
.page-container {
  display: flex;
  min-height: 100vh;
}

.sidebar {
  width: 200px;
  background-color: #050505;
  color: white;
  padding: 20px;
  height: 100vh;
}

.sidebar ul {
  list-style-type: none;
  padding: 0;
}

.sidebar li {
  margin: 10px 0;
  padding: 8px;
  cursor: pointer;
  border-radius: 4px;
}

.sidebar li:hover {
  background-color: #444;
}

.sidebar li.active {
  background-color: #393939;
}

.content {
  flex: 1;
  padding: 20px;
}

.form-container-profile {
  background-color: #050505;
  padding: 15px;
  border-radius: 8px;
  width: 100%;
  max-width: 600px;
  height: 370px;
  margin: 0 auto;
  position: relative;
}

.form-container-account {
  background-color: #050505;
  padding: 15px;
  border-radius: 8px;
  width: 100%;
  max-width: 600px;
  height: 340px;
  margin: 0 auto;
  position: relative;
}

.form-container-theme {
  background-color: #050505;
  padding: 15px;
  border-radius: 8px;
  width: 100%;
  max-width: 600px;
  height: 100px;
  margin: 0 auto;
  position: relative;
}

.form-title {
  font-size: 24px;
  color: white;
  margin-bottom: 10px;
}

.form-field {
  margin-bottom: 10px;
}

.form-field label {
  font-weight: bold;
  color: white;
}

.form-field input,
.form-field textarea {
  width: 100%;
  padding: 6px;
  margin-top: 4px;
  border-radius: 4px;
  border: 1px solid #7f8c8d;
  background-color: white;
  color: black;
}

.form-field input::placeholder,
.form-field textarea::placeholder {
  color: black;
  opacity: 0.5;
}

.buttons {
  position: absolute;
  bottom: 10px;
  left: 15px;
}

.submit-button {
  background-color: white;
  color: black;
  padding: 10px 20px;
  border-radius: 4px;
  cursor: pointer;
  width: 150px;
}

.submit-button:hover {
  background-color: #bdc3c7;
}

.dark-theme {
  background-color: #050505;
  color: white;
}

.light-theme {
  background-color: #ecf0f1;
  color: black;
}

.dark-theme .form-field input,
.dark-theme .form-field textarea {
  background-color: #333;
  color: white;
}

.dark-theme .form-field input::placeholder,
.dark-theme .form-field textarea::placeholder {
  color: white; /* Cor dos placeholders no tema escuro */
}

.switch-container {
  display: flex;
  align-items: center;
}

.switch-label {
  margin-right: 10px;
  color: white;
}

.switch-container input {
  width: 40px;
  height: 20px;
  border-radius: 20px;
  background-color: #7f8c8d;
  position: relative;
  appearance: none;
  cursor: pointer;
}

.switch-container input:checked {
  background-color: #16a085;
}

.switch-container input:before {
  content: '';
  position: absolute;
  top: 2px;
  left: 2px;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background-color: white;
  transition: 0.3s;
}

.home-button {
  background-color: 050505;
  color: rgb(255, 255, 255);
  padding: 8px;
  border-radius: 4px;
  cursor: pointer;
  text-align: center;
  position: absolute;
  bottom: 20px;
  left: 20px;
}

.home-button:hover {
  background-color: #444;
}

.switch-container input:checked:before {
  left: 22px;
}
</style>
