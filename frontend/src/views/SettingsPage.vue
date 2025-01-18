<script setup lang="ts">
import { ref } from 'vue'
import { Button } from '@/components/ui/button'
import { Separator } from '@/components/ui/separator'
import { Switch } from '@/components/ui/switch'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { User, CreditCard, Bell, Shield, Link, Globe, Github, Mail } from 'lucide-vue-next'

interface Tab {
  id: string
  name: string
  icon: typeof User // Using one of the icons as a type reference
}

const tabs: Tab[] = [
  { id: 'account', name: 'Account Information', icon: User },
  { id: 'billing', name: 'Billing', icon: CreditCard },
  { id: 'notifications', name: 'Notifications', icon: Bell },
  { id: 'privacy', name: 'Privacy & Security', icon: Shield },
  { id: 'connected', name: 'Connected Accounts', icon: Link },
  { id: 'language', name: 'Language & Region', icon: Globe },
]

const activeTab = ref('account')
</script>

<template>
  <div class="min-h-screen bg-[#030712] text-white">
    <div class="container mx-auto p-8">
      <h1 class="text-4xl font-bold mb-8">Settings</h1>

      <div class="flex gap-8">
        <!-- Sidebar -->
        <div class="w-64">
          <nav class="space-y-3">
            <Button
              v-for="tab in tabs"
              :key="tab.id"
              variant="ghost"
              :class="[
                'w-full justify-start text-left',
                activeTab === tab.id
                  ? 'bg-[#0f1629] text-white'
                  : 'text-[#969696] hover:bg-[#0f1629] hover:text-white',
              ]"
              @click="activeTab = tab.id"
            >
              <component :is="tab.icon" class="mr-2 h-5 w-5" />
              {{ tab.name }}
            </Button>
          </nav>

          <div class="mt-auto pt-8">
            <h1
              class="font-bold bg-gradient-to-r from-[#6D28D9] to-white bg-clip-text text-transparent tracking-tight text-4xl font-bold text-center"
            >
              PICTURAS
            </h1>
          </div>
        </div>

        <!-- Main Content -->
        <div class="flex-1 bg-[#0f1629] rounded-2xl p-8">
          <!-- Account Information -->
          <div v-if="activeTab === 'account'" class="space-y-6">
            <h2 class="text-2xl font-semibold mb-6">Account Information</h2>

            <div class="space-y-5">
              <div class="flex items-center justify-between">
                <div>
                  <h3 class="font-medium">Name</h3>
                  <p class="text-[#969696]">email@gmail.com</p>
                </div>
                <Button variant="link" class="text-[#DD3592]">Edit</Button>
              </div>

              <Separator class="bg-white/10" />

              <div class="flex items-center justify-between">
                <div>
                  <h3 class="font-medium">Password</h3>
                  <p class="text-[#969696]">Last Edit 1 month ago</p>
                </div>
                <Button variant="link" class="text-[#DD3592]">Edit</Button>
              </div>

              <Separator class="bg-white/10" />

              <div class="flex items-center justify-between">
                <div>
                  <h3 class="font-medium">Two-Factor Authentication</h3>
                  <p class="text-[#969696]">Enable</p>
                </div>
                <Button variant="link" class="text-[#DD3592]">Edit</Button>
              </div>
            </div>
          </div>

          <!-- Billing -->
          <div v-if="activeTab === 'billing'" class="space-y-6">
            <h2 class="text-2xl font-semibold mb-6">Billing</h2>

            <div class="space-y-5">
              <div class="flex items-center justify-between">
                <div>
                  <h3 class="font-medium">Current Plan</h3>
                  <p class="text-[#969696]">Premium - 9.99 / Month</p>
                </div>
                <Button variant="link" class="text-[#DD3592]">Change Plan</Button>
              </div>

              <Separator class="bg-white/10" />

              <div class="flex items-center justify-between">
                <div>
                  <h3 class="font-medium">Payment Method</h3>
                  <p class="text-[#969696]">Master Card - **** **** **** 1234</p>
                </div>
                <Button variant="link" class="text-[#DD3592]">Edit</Button>
              </div>

              <Separator class="bg-white/10" />

              <div class="flex items-center justify-between">
                <div>
                  <h3 class="font-medium">Address</h3>
                  <div class="text-[#969696] space-y-1">
                    <p>Rua do JMF</p>
                    <p>Porta GB, Resto Chao Esq</p>
                    <p>Braga, Portugal</p>
                    <p>4700-123</p>
                  </div>
                </div>
                <Button variant="link" class="text-[#DD3592]">Edit</Button>
              </div>
            </div>
          </div>

          <!-- Notifications -->
          <div v-if="activeTab === 'notifications'" class="space-y-6">
            <h2 class="text-2xl font-semibold mb-6">Notifications</h2>

            <div class="space-y-6">
              <div class="flex items-center justify-between">
                <div>
                  <h3 class="font-medium">Email Notifications</h3>
                  <p class="text-[#969696]">Receive updates about your projects</p>
                </div>
                <Switch />
              </div>

              <Separator class="bg-white/10" />

              <div class="flex items-center justify-between">
                <div>
                  <h3 class="font-medium">Marketing Emails</h3>
                  <p class="text-[#969696]">Receive offers and announcements</p>
                </div>
                <Switch />
              </div>
            </div>
          </div>

          <!-- Privacy -->
          <div v-if="activeTab === 'privacy'" class="space-y-6">
            <h2 class="text-2xl font-semibold mb-6">Privacy & Security</h2>

            <div class="space-y-6">
              <div class="flex items-center justify-between">
                <div>
                  <h3 class="font-medium">Profile Visibility</h3>
                  <p class="text-[#969696]">Control who can see your profile</p>
                </div>
                <Select defaultValue="public">
                  <SelectTrigger class="w-38 bg-[#0f1629]">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="public">Public</SelectItem>
                    <SelectItem value="private">Private</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <Separator class="bg-white/10" />

              <div class="flex items-center justify-between">
                <div>
                  <h3 class="font-medium">Active Sessions</h3>
                  <p class="text-[#969696]">Manage your active login sessions</p>
                </div>
                <Button variant="link" class="text-[#DD3592]">View</Button>
              </div>
            </div>
          </div>

          <!-- Connected Accounts -->
          <div v-if="activeTab === 'connected'" class="space-y-6">
            <h2 class="text-2xl font-semibold mb-6">Connected Accounts</h2>

            <div class="space-y-5">
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                  <Github class="h-6 w-6" />
                  <div>
                    <h3 class="font-medium">GitHub</h3>
                    <p class="text-[#969696]">Not connected</p>
                  </div>
                </div>
                <Button>Connect</Button>
              </div>

              <Separator class="bg-white/10" />

              <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                  <Mail class="h-6 w-6" />
                  <div>
                    <h3 class="font-medium">Google</h3>
                    <p class="text-[#969696]">Connected</p>
                  </div>
                </div>
                <Button>Disconnect</Button>
              </div>
            </div>
          </div>

          <!-- Language -->
          <div v-if="activeTab === 'language'" class="space-y-6">
            <h2 class="text-2xl font-semibold mb-6">Language & Region</h2>

            <div class="space-y-6">
              <div class="flex items-center justify-between">
                <div>
                  <h3 class="font-medium">Language</h3>
                  <p class="text-[#969696]">Select your preferred language</p>
                </div>
                <Select defaultValue="en">
                  <SelectTrigger class="w-32 bg-[#0f1629]">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="en">English</SelectItem>
                    <SelectItem value="pt">Portuguese</SelectItem>
                    <SelectItem value="es">Spanish</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <Separator class="bg-white/10" />

              <div class="flex items-center justify-between">
                <div>
                  <h3 class="font-medium">Time Zone</h3>
                  <p class="text-[#969696]">Set your local time zone</p>
                </div>
                <Select defaultValue="utc">
                  <SelectTrigger class="w-32 bg-[#0f1629]">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="utc">UTC</SelectItem>
                    <SelectItem value="est">EST (UTC-5)</SelectItem>
                    <SelectItem value="pst">PST (UTC-8)</SelectItem>
                  </SelectContent>
                </Select>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
