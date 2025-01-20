<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useAuth } from '@/lib/auth'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { CameraIcon } from 'lucide-vue-next'
import { useUserStore } from '@/stores/user'
import router from '@/router'

const { registerUser, loginUser, getUser } = useAuth()

const isLogin = ref(false)
const username = ref('')
const email = ref('')
const password = ref('')

const toggleForm = () => {
  isLogin.value = !isLogin.value
}

const isSubmitting = ref(false)

interface RegisterFormData {
  email: string
  username: string
  password: string
}

interface RegisterFormErrors {
  email?: string
  username?: string
  password?: string
}

const registerForm = reactive<RegisterFormData>({
  email: '',
  username: '',
  password: '',
})

const errors = reactive<RegisterFormErrors>({})

const validateRegisterForm = (): boolean => {
  errors.email = ''
  errors.username = ''
  errors.password = ''

  let isValid = true

  if (!registerForm.email) {
    errors.email = 'Email is required'
    isValid = false
  } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(registerForm.email)) {
    errors.email = 'Invalid email format'
    isValid = false
  }

  if (!registerForm.username) {
    errors.username = 'Username is required'
    isValid = false
  }

  if (!registerForm.password) {
    errors.password = 'Password is required'
    isValid = false
  } else if (registerForm.password.length < 8) {
    errors.password = 'Password must be at least 8 characters'
    isValid = false
  }

  return isValid
}

const handleRegister = async () => {
  if (!validateRegisterForm()) return

  isSubmitting.value = true

  try {
    const response = await registerUser(registerForm.username, registerForm.email, registerForm.password);

    if (!response) {
      alert('Email already exists')
      isSubmitting.value = false
      return
    }

    useUserStore().login(
      response.name,
      response.email,
      false,
      response.uuid
    );

    router.push('/projects');
  } catch (error) {
    console.error('Register process failed:', error);
  } finally {
    isSubmitting.value = false;
  }
}

interface LoginFormData {
  email: string
  password: string
}

interface LoginFormErrors {
  email?: string
  password?: string
}

const loginForm = reactive<LoginFormData>({
  email: '',
  password: '',
})

const loginErrors = reactive<LoginFormErrors>({})

const validateLoginForm = (): boolean => {
  loginErrors.email = ''
  loginErrors.password = ''

  console.log(loginForm.email, loginForm.password)

  let isValid = true

  if (!loginForm.email) {
    loginErrors.email = 'Email is required'
    isValid = false
  } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(loginForm.email)) {
    loginErrors.email = 'Invalid email format'
    isValid = false
  }

  if (!loginForm.password) {
    loginErrors.password = 'Password is required'
    isValid = false
  }

  if (loginForm.password.length < 8) {
    loginErrors.password = 'Password must be at least 8 characters'
    isValid = false
  }
  console.log(loginErrors)
  return isValid
}

const handleLogin = async () => {
  if (!validateLoginForm()) return

  isSubmitting.value = true

  try {
    await loginUser(loginForm.email, loginForm.password);

    const response = await getUser();

    if (!response) {
      alert('Invalid email or password');
      isSubmitting.value = false;
      return;
    }
  
    useUserStore().login(
      response.name,
      response.email,
      false,
      response.uuid
    );

    router.push('/projects');
  } catch (error) {
    console.error('Login process failed:', error);
  } finally {
    isSubmitting.value = false;
  }
}

onMounted(() => {
  animateBackground()
})

const animateBackground = () => {
  const canvas = document.getElementById('bgCanvas') as HTMLCanvasElement
  const ctx = canvas.getContext('2d')
  if (!ctx) return

  canvas.width = window.innerWidth
  canvas.height = window.innerHeight

  const particles: { x: number; y: number; size: number; speedX: number; speedY: number }[] = []
  const particleCount = 100

  for (let i = 0; i < particleCount; i++) {
    particles.push({
      x: Math.random() * canvas.width,
      y: Math.random() * canvas.height,
      size: Math.random() * 5 + 1,
      speedX: Math.random() * 3 - 1.5,
      speedY: Math.random() * 3 - 1.5,
    })
  }

  function animate() {
    if (!ctx) return
    ctx.clearRect(0, 0, canvas.width, canvas.height)
    particles.forEach((particle) => {
      ctx.beginPath()
      ctx.arc(particle.x, particle.y, particle.size, 0, Math.PI * 2)
      ctx.fillStyle = '#6D28D9'
      ctx.fill()

      particle.x += particle.speedX
      particle.y += particle.speedY

      if (particle.x < 0 || particle.x > canvas.width) particle.speedX *= -1
      if (particle.y < 0 || particle.y > canvas.height) particle.speedY *= -1
    })
    requestAnimationFrame(animate)
  }

  animate()
}
</script>

<template>
  <div class="min-h-screen bg-[#030712] text-white relative overflow-hidden">
    <canvas id="bgCanvas" class="absolute inset-0 z-0"></canvas>

    <div
      class="container relative z-10 flex flex-col items-center justify-center min-h-screen px-4"
    >
      <div class="text-center mb-8">
        <h1
          class="text-5xl font-bold mb-2 bg-gradient-to-r from-[#6D28D9] to-white bg-clip-text text-transparent tracking-tight"
        >
          Welcome to PictuRAS
        </h1>
        <p class="text-xl text-[#969696]">
          Unleash your creativity with our powerful image editing tools
        </p>
      </div>

      <div v-if="!useUserStore().loggedIn()" class="bg-[#0f1629] p-8 rounded-3xl shadow-xl w-full max-w-md">
        <div class="flex items-center justify-center mb-6">
          <CameraIcon class="w-12 h-12 text-[#6D28D9]" />
          <span
            class="text-2xl font-bold ml-2 bg-gradient-to-r from-[#6D28D9] to-white bg-clip-text text-transparent tracking-tight"
            >PictuRAS</span
          >
        </div>

        <div v-if="!isLogin">
          <form @submit.prevent="handleRegister" class="space-y-4">
            <Input
              type="text "
              class="rounded-2xl"
              placeholder="Username"
              v-model="registerForm.username"
              required
            />
            <Input type="email"
              class="rounded-2xl"
              placeholder="Email" 
              v-model="registerForm.email" 
              required 
            />
            <Input
              type="password"
              class="rounded-2xl"
              placeholder="Password"
              v-model="registerForm.password"
              required
            />
            <Button type="submit" class="w-full rounded-3xl bg-[#6D28D9] hover:bg-[#5b21b6]">
              Register
            </Button>
          </form>
          <div class="text-center mt-4">
            <p class="text-sm text-[#969696]">
              Already have an account?
              <a @click="toggleForm" href="#" class="text-[#6D28D9] hover:underline">Login</a>
            </p>
          </div>
        </div>
        <div v-else>
          <form @submit.prevent="handleLogin" class="space-y-4">
            <Input
              type="email"
              class="rounded-2xl"
              placeholder="Email"
              v-model="loginForm.email"
              required
            />
            <Input
              type="password"
              class="rounded-2xl"
              placeholder="Password"
              v-model="loginForm.password"
              required
            />
            <Button type="submit" class="w-full bg-[#6D28D9] hover:bg-[#5b21b6]">
              Login
            </Button>
          </form>
          <div class="text-center mt-4">
            <p class="text-sm text-[#969696]">
              Don't have an account?
              <a @click="toggleForm" href="#" class="text-[#6D28D9] hover:underline">Register</a>
            </p>
            <router-link to="" class="text-[#6D28D9] hover:underline">
              Forgot your password?
            </router-link>
          </div>
        </div>
      </div>

      <div class="mt-8 text-center">
        <blockquote class="text-lg italic text-[#969696]">
          "PictuRAS revolutionized how I edit images. It's easy to use and packed with powerful
          features."
        </blockquote>
        <footer class="mt-2 text-sm text-[#6D28D9]">- JMF</footer>
      </div>
    </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
}
</style>
