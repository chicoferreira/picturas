<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAuth } from '@/lib/auth'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { CameraIcon } from 'lucide-vue-next'

const { registerUser, loginUser } = useAuth()

const isLogin = ref(false)
const username = ref('')
const email = ref('')
const password = ref('')

const toggleForm = () => {
  isLogin.value = !isLogin.value
}

const handleRegister = async () => {
  await registerUser(username.value, email.value, password.value)
  toggleForm()
}

const handleLogin = async () => {
  await loginUser(email.value, password.value)
  toggleForm()
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

      <div class="bg-[#0f1629] p-8 rounded-3xl shadow-xl w-full max-w-md">
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
              v-model="username"
              required
            />
            <Input type="email" class="rounded-2xl" placeholder="Email" v-model="email" required />
            <Input
              type="password"
              class="rounded-2xl"
              placeholder="Password"
              v-model="password"
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
              v-model="email"
              required
            />
            <Input
              type="password"
              class="rounded-2xl"
              placeholder="Password"
              v-model="password"
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
