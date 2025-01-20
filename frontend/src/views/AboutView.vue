<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import { Card, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { CameraIcon, EditIcon, LayersIcon, UserIcon } from 'lucide-vue-next'

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
  <div class="min-h-screen bg-[#030712] text-white relative overflow-hidden">
    <canvas id="bgCanvas" class="absolute inset-0 z-0"></canvas>

    <div class="container mx-auto px-4 py-16 relative z-10">
      <header class="text-center mb-16">
        <h1
          class="text-5xl font-bold bg-gradient-to-r from-[#6D28D9] to-white bg-clip-text text-transparent tracking-tight mb-4"
        >
          PICTURAS
        </h1>
        <p class="text-xl text-[#969696]">Unleash Your Creativity with Powerful Image Editing</p>
      </header>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-8 mb-16">
        <Card class="bg-[#0f1629] bg-opacity-80 backdrop-blur-sm border-0">
          <CardContent class="p-6">
            <h2 class="text-2xl font-semibold mb-4 flex items-center">
              <CameraIcon class="mr-2 text-[#6D28D9]" />
              About PICTURAS
            </h2>
            <p class="text-[#969696]">
              PICTURAS is a cutting-edge image editing platform designed for both amateur
              enthusiasts and professional photographers. Our mission is to provide powerful,
              intuitive tools that allow users to transform their visual ideas into stunning
              realities.
            </p>
          </CardContent>
        </Card>

        <Card class="bg-[#0f1629] bg-opacity-80 backdrop-blur-sm border-0">
          <CardContent class="p-6">
            <h2 class="text-2xl font-semibold mb-4 flex items-center">
              <EditIcon class="mr-2 text-[#6D28D9]" />
              Key Features
            </h2>
            <ul class="list-disc list-inside text-[#969696]">
              <li>Advanced photo editing tools</li>
              <li>AI-powered enhancements</li>
              <li>Collaborative project workspaces</li>
              <li>Cloud storage and syncing</li>
              <li>Extensive template library</li>
            </ul>
          </CardContent>
        </Card>
      </div>

      <div class="text-center mb-16">
        <h2 class="text-3xl font-semibold mb-8">Why Choose PICTURAS?</h2>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
          <div class="flex flex-col items-center">
            <LayersIcon class="h-12 w-12 text-[#6D28D9] mb-4" />
            <h3 class="text-xl font-semibold mb-2">Powerful Editing</h3>
            <p class="text-[#969696]">
              Access professional-grade tools for precise image manipulation
            </p>
          </div>
          <div class="flex flex-col items-center">
            <UserIcon class="h-12 w-12 text-[#6D28D9] mb-4" />
            <h3 class="text-xl font-semibold mb-2">User-Friendly</h3>
            <p class="text-[#969696]">
              Intuitive interface suitable for beginners and experts alike
            </p>
          </div>
          <div class="flex flex-col items-center">
            <CameraIcon class="h-12 w-12 text-[#6D28D9] mb-4" />
            <h3 class="text-xl font-semibold mb-2">Stunning Results</h3>
            <p class="text-[#969696]">Create professional-quality images with ease</p>
          </div>
        </div>
      </div>

      <div class="text-center">
        <h2 class="text-3xl font-semibold mb-8">Ready to Transform Your Images?</h2>
        <router-link to="/">
          <Button class="bg-[#6D28D9] hover:bg-[#5b21b6] text-white px-8 py-3 rounded-full text-lg">
            Get Started Now
          </Button>
        </router-link>
      </div>
    </div>
  </div>
</template>
