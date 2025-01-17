<template>
  <div class="min-h-screen flex items-center justify-center bg-[#030712] p-6">
    <div class="w-full max-w-md space-y-8">
      <!-- Added Logo -->
      <div class="text-center">
        <h1
          class="text-5xl font-bold bg-gradient-to-r from-[#6D28D9] to-white bg-clip-text text-transparent tracking-tight"
        >
          PICTURAS
        </h1>
      </div>

      <div class="space-y-2 text-center">
        <h2 class="text-3xl font-semibold tracking-tight text-white">Create Account</h2>
        <p class="text-[#969696]">
          Enter your email and a secure password to create your new account
        </p>
      </div>

      <form @submit.prevent="handleSubmit" class="space-y-4">
        <div class="space-y-2">
          <label for="email" class="text-sm font-medium text-white"> Email </label>
          <input
            v-model="form.email"
            id="email"
            type="email"
            required
            class="w-full px-3 py-2 rounded-md border border-gray-800 bg-transparent text-white focus:outline-none focus:ring-2 focus:ring-[#6D28D9] focus:border-transparent"
            :class="{ 'border-red-500': errors.email }"
          />
          <p v-if="errors.email" class="text-sm text-red-500">{{ errors.email }}</p>
        </div>

        <div class="space-y-2">
          <label for="username" class="text-sm font-medium text-white"> Username </label>
          <input
            v-model="form.username"
            id="username"
            type="text"
            required
            class="w-full px-3 py-2 rounded-md border border-gray-800 bg-transparent text-white focus:outline-none focus:ring-2 focus:ring-[#6D28D9] focus:border-transparent"
            :class="{ 'border-red-500': errors.username }"
          />
          <p v-if="errors.username" class="text-sm text-red-500">{{ errors.username }}</p>
        </div>

        <div class="space-y-2">
          <label for="password" class="text-sm font-medium text-white"> Password </label>
          <input
            v-model="form.password"
            id="password"
            type="password"
            required
            class="w-full px-3 py-2 rounded-md border border-gray-800 bg-transparent text-white focus:outline-none focus:ring-2 focus:ring-[#6D28D9] focus:border-transparent"
            :class="{ 'border-red-500': errors.password }"
          />
          <p v-if="errors.password" class="text-sm text-red-500">{{ errors.password }}</p>
        </div>

        <button
          type="submit"
          class=" w-full px-4 py-2 bg-[#6D28D9] text-white rounded-full hover:bg-[#5b21b6] focus:outline-none focus:ring-2 focus:ring-[#6D28D9] focus:ring-offset-2 focus:ring-offset-[#030712] transition-colors"
          :disabled="isSubmitting"
        >
          {{ isSubmitting ? 'Creating Account...' : 'Create Account' }}
        </button>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useUserStore } from '@/stores/user'
import router from '@/router'

interface FormData {
  email: string
  username: string
  password: string
}

interface FormErrors {
  email?: string
  username?: string
  password?: string
}

const form = reactive<FormData>({
  email: '',
  username: '',
  password: '',
})

const errors = reactive<FormErrors>({})
const isSubmitting = ref(false)

const validateForm = (): boolean => {
  errors.email = ''
  errors.username = ''
  errors.password = ''

  let isValid = true

  if (!form.email) {
    errors.email = 'Email is required'
    isValid = false
  } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(form.email)) {
    errors.email = 'Invalid email format'
    isValid = false
  }

  if (!form.username) {
    errors.username = 'Username is required'
    isValid = false
  }

  if (!form.password) {
    errors.password = 'Password is required'
    isValid = false
  } else if (form.password.length < 8) {
    errors.password = 'Password must be at least 8 characters'
    isValid = false
  }

  return isValid
}

const handleSubmit = async () => {
  if (!validateForm()) return

  isSubmitting.value = true

  try {
    // TODO alterar para usar API de dados
    // Simulate API call
    await new Promise((resolve) => setTimeout(resolve, 1000))
    console.log('Form submitted:', form)
    // Reset form after successful submission
    useUserStore().login(form.username,form.email, 'token', true, '1')
    //useUserStore().login(form.username,form.email, 'token', false, '2')
    console.log('Loggin in:')
    router.push('/projects')
  } catch (error) {
    console.error('Error submitting form:', error)
  } finally {
    isSubmitting.value = false
  }
}

onMounted (() => {
  useUserStore().logout()
})
</script>