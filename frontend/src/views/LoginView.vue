<template>
  <div class="min-h-screen flex flex-col items-center justify-center bg-[#030712] p-6">
    <div class="w-full max-w-md space-y-8">
      <!-- Logo -->
      <div class="text-center">
        <h1
          class="text-5xl font-bold bg-gradient-to-r from-[#6D28D9] to-white bg-clip-text text-transparent tracking-tight"
        >
          PICTURAS
        </h1>
      </div>

      <Card class="border-0 bg-[#030712]">
        <CardHeader>
          <CardTitle class="text-3xl font-semibold tracking-tight text-white text-center">
            Login
          </CardTitle>
          <CardDescription class="text-[#969696] text-center">
            Enter your email and password to login to your account
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form @submit.prevent="handleSubmit" class="space-y-4">
            <div class="space-y-2">
              <Label for="email" class="text-white">Email</Label>
              <Input
                v-model="form.email"
                id="email"
                type="email"
                placeholder="m@example.com"
                required
                :class="{ 'border-red-500': errors.email }"
                class="bg-transparent text-white border-gray-800"
              />
              <p v-if="errors.email" class="text-sm text-red-500">{{ errors.email }}</p>
            </div>

            <div class="space-y-2">
              <Label for="password" class="text-white">Password</Label>
              <Input
                v-model="form.password"
                id="password"
                type="password"
                required
                :class="{ 'border-red-500': errors.password }"
                class="bg-transparent text-white border-gray-800"
              />
              <p v-if="errors.password" class="text-sm text-red-500">{{ errors.password }}</p>
            </div>

            <Button
              type="submit"
              :disabled="isSubmitting"
              class="w-full bg-[#6D28D9] hover:bg-[#5b21b6] transition-colors"
            >
              {{ isSubmitting ? 'Logging in...' : 'Login' }}
            </Button>
          </form>
        </CardContent>
      </Card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui/card'
import { Label } from '@/components/ui/label'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'

interface FormData {
  email: string
  password: string
}

interface FormErrors {
  email?: string
  password?: string
}

const form = reactive<FormData>({
  email: '',
  password: '',
})

const errors = reactive<FormErrors>({})
const isSubmitting = ref(false)

const validateForm = (): boolean => {
  errors.email = ''
  errors.password = ''

  let isValid = true

  if (!form.email) {
    errors.email = 'Email is required'
    isValid = false
  } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(form.email)) {
    errors.email = 'Invalid email format'
    isValid = false
  }

  if (!form.password) {
    errors.password = 'Password is required'
    isValid = false
  }

  return isValid
}

const handleSubmit = async () => {
  if (!validateForm()) return

  isSubmitting.value = true

  try {
    // Simulate API call
    await new Promise((resolve) => setTimeout(resolve, 1000))
    console.log('Form submitted:', form)
    // Reset form after successful submission
    form.email = ''
    form.password = ''
  } catch (error) {
    console.error('Error submitting form:', error)
  } finally {
    isSubmitting.value = false
  }
}
</script>
