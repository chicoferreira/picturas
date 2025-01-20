<script setup lang="ts">
import { ref, reactive, onMounted, onUnmounted } from 'vue'
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui/card'
import { Label } from '@/components/ui/label'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { useAuth } from '@/lib/auth'
import router from '@/router'

const { changePassword } = useAuth()

interface FormData {
  oldPassword: string
  newPassword: string
}

interface FormErrors {
  oldPassword: string
  newPassword: string
}

const form = reactive<FormData>({
  oldPassword: '',
  newPassword: ''
})

const errors = reactive<FormErrors>({
  oldPassword: '',
  newPassword: ''
})
const isSubmitting = ref(false)

const validateForm = (): boolean => {
  errors.oldPassword = ''
  errors.newPassword = ''

  let isValid = true

  if (!form.oldPassword) {
    errors.oldPassword = 'Old Password is required'
    isValid = false
  }

  if (!form.newPassword) {
    errors.newPassword = 'New Password is required'
    isValid = false
  } else if (form.newPassword.length < 8) {
    errors.newPassword = 'New Password must be at least 8 characters'
    isValid = false
  }

  return isValid
}

const handleSubmit = async () => {
  if (!validateForm()) return

  isSubmitting.value = true

  try {
    await changePassword(form.oldPassword, form.newPassword);


    router.push('/settings');
  } catch (error) {
    console.error('Process failed:', error);
  } finally {
    isSubmitting.value = false;
  }
}

// Background animation
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

const handleResize = () => {
  const canvas = document.getElementById('bgCanvas') as HTMLCanvasElement
  canvas.width = window.innerWidth
  canvas.height = window.innerHeight
}

onMounted(() => {
  animateBackground()
  window.addEventListener('resize', handleResize)
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
})
</script>

<template>
  <div class="min-h-screen flex flex-col items-center justify-center bg-[#030712] p-6 relative">
    <canvas id="bgCanvas" class="absolute inset-0 z-0"></canvas>
    <div class="w-full max-w-md space-y-8 relative z-10">
      <div class="text-center">
        <h1
          class="text-5xl font-bold bg-gradient-to-r from-[#6D28D9] to-white bg-clip-text text-transparent tracking-tight"
        >
          PICTURAS
        </h1>
      </div>

      <Card class="border-0 bg-[#030712] bg-opacity-80 backdrop-blur-sm">
        <CardHeader>
          <CardTitle class="text-3xl font-semibold tracking-tight text-white text-center">
            Change your password
          </CardTitle>
          <CardDescription class="text-[#969696] text-center">
            Enter a new password
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form @submit.prevent="handleSubmit" class="space-y-4">
            <div class="space-y-2">
              <Label for="oldPassword" class="text-white">Old Password</Label>
              <Input
                v-model="form.oldPassword"
                id="oldPassword"
                type="password"
                required
                :class="{ 'border-red-500': errors.oldPassword }"
                class="bg-transparent text-white border-gray-800 rounded-2xl"
              />
              <p v-if="errors.oldPassword" class="text-sm text-red-500">{{ errors.oldPassword }}</p>
            </div>

            <div class="space-y-2">
              <Label for="newPassword" class="text-white">New Password</Label>
              <Input
                v-model="form.newPassword"
                id="newPassword"
                type="password"
                required
                :class="{ 'border-red-500': errors.newPassword }"
                class="bg-transparent text-white border-gray-800 rounded-2xl"
              />
              <p v-if="errors.newPassword" class="text-sm text-red-500">{{ errors.newPassword }}</p>
            </div>

            <Button
              type="submit"
              :disabled="isSubmitting"
              class="w-full bg-[#6D28D9] hover:bg-[#5b21b6] transition-colors rounded-3xl"
            >
              {{ isSubmitting ? 'Logging in...' : 'Change Password' }}
            </Button>
          </form>
        </CardContent>
      </Card>
    </div>
  </div>
</template>
