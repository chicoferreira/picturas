<script setup lang="ts">
import { ref } from 'vue'
import { useAuth } from '@/lib/auth';

const { registerUser, loginUser } = useAuth();

const isLogin = ref(false);

const username = ref('');
const email = ref('');
const password = ref('');

const toggleForm = () => {
  isLogin.value = !isLogin.value;
};

const handleRegister = async () => {
  await registerUser(username.value, email.value, password.value);
  toggleForm();
};

const handleLogin = async () => {
  await loginUser(email.value, password.value);
  toggleForm();
};
</script>

<template>
  <div class="md:hidden">
    <VPImage
      alt="PictuRAS - Image Editing"
      width="1280"
      height="1214" class="block" :image="{
        dark: '/examples/picturas-dark.png',
        light: '/examples/picturas-light.png',
      }"
    />
  </div>

  <div class="container relative hidden h-[800px] flex-col items-center justify-center md:grid lg:max-w-none lg:grid-cols-2 lg:px-0">
    <div class="relative hidden h-full flex-col bg-muted p-10 text-white dark:border-r lg:flex">
      <div class="absolute inset-0 bg-zinc-900" />
      <div class="relative z-20 flex items-center text-lg font-medium">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinecap="round"
          strokeLinejoin="round"
          class="mr-2 h-6 w-6"
        >
          <path d="M15 6v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3" />
        </svg>
        PictuRAS
      </div>
      <div class="relative z-20 mt-auto">
        <blockquote class="space-y-2">
          <p class="text-lg">
            &ldquo;PictuRAS revolutionized how I edit images. It's easy to use and packed with powerful features.&rdquo;
          </p>
          <footer class="text-sm">
            JMF
          </footer>
        </blockquote>
      </div>
    </div>
    <div class="lg:p-8">
      <div class="mx-auto flex w-full flex-col justify-center space-y-6 sm:w-[350px]">
        <div class="flex flex-col space-y-2 text-center">
          <h1 class="text-3xl font-semibold tracking-tight">
            Bem-vindo ao PictuRAS
          </h1>
          <p class="text-sm text-muted-foreground">
            A melhor ferramenta de edição de imagens que você pode usar para criar, editar e transformar suas fotos.
          </p>
        </div>

        <!-- Formulário de Registo ou Login -->
        <div v-if="!isLogin">
          <form @submit.prevent="handleRegister" class="space-y-4">
            <input type="text" name="username" placeholder="Usuário" v-model="username" class="w-full px-4 py-2 border rounded-lg text-black" required />
            <input type="email" name="email" placeholder="Email" v-model="email" class="w-full px-4 py-2 border rounded-lg text-black" required />
            <input type="password" name="password" placeholder="Senha" v-model="password" class="w-full px-4 py-2 border rounded-lg text-black" required />
            <button type="submit" class="w-full px-8 py-3 text-center text-white bg-black hover:bg-gray-600 rounded-lg">
              Registar
            </button>
          </form>
          <div class="text-center mt-4">
            <p class="text-sm text-muted-foreground">
              Já tem uma conta? 
              <a @click="toggleForm" href="#" class="text-blue-500 hover:underline">Login</a>
            </p>
          </div>
        </div>

        <div v-else>
          <form @submit.prevent="handleLogin" class="space-y-4">
            <input type="email" name="email" placeholder="Email" v-model="email" class="w-full px-4 py-2 border rounded-lg text-black" required />
            <input type="password" name="password" placeholder="Senha" v-model="password" class="w-full px-4 py-2 border rounded-lg text-black" required />
            <button type="submit" class="w-full px-8 py-3 text-center text-white bg-black hover:bg-gray-600 rounded-lg">
              Login
            </button>
          </form>
          <div class="text-center mt-4">
            <p class="text-sm text-muted-foreground">
              Não tem uma conta? 
              <a @click="toggleForm" href="#" class="text-blue-500 hover:underline">Registe-se</a>
            </p>
          </div>
        </div>

      </div>
    </div>
  </div>
</template>
