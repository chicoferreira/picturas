<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { SearchIcon, PlusIcon, PencilIcon, Trash2, ImageUp } from 'lucide-vue-next'
import { useUserStore } from '@/stores/user'
import { useAuth } from '@/lib/auth'
import { Card, CardFooter } from '@/components/ui/card'
const API_BASE = import.meta.env.VITE_API_BASE  || 'https://f.primecog.com/api/v1';
const router = useRouter()
const userStore = useUserStore()

console.log("API_BASE = " + API_BASE)

const { authFetch } = useAuth()

interface Project {
  id: string
  name: string
  imageUrl?: string
  lastEdited: number
  firstImageId?: string
}

interface ApiProject {
  id: string
  name: string
  user_id: string
  created_at: string
  updated_at: string
}

interface ApiImage {
  id: string
  name: string
  project_id: string
}

const searchQuery = ref('')
const projects = ref<Project[]>([])

const userName = computed(() => userStore.name)
const userEmail = computed(() => userStore.email)

function daysSince(dateString: string): number {
  const updatedDate = new Date(dateString)
  const now = new Date()
  const diffMs = now.getTime() - updatedDate.getTime()
  return Math.floor(diffMs / (1000 * 60 * 60 * 24))
}

async function getProjectImage(projectId: string, imageId: string): Promise<string> {
  try {
    const response = await authFetch(
      API_BASE + `/projects/${projectId}/images/${imageId}`,
      {
        method: 'GET',
        credentials: 'include',
      }
    )
    
    if (!response.ok) throw new Error('Failed to fetch image')
    
    const blob = await response.blob()
    return URL.createObjectURL(blob)
  } catch (error) {
    console.error('Error fetching image:', error)
    return '/placeholder.svg?height=400&width=600'
  }
}

async function getProjectImages(projectId: string): Promise<string | undefined> {
  try {
    const response = await authFetch(
      API_BASE + `/projects/${projectId}/images`,
      {
        method: 'GET',
        credentials: 'include',
      }
    )
    
    if (!response.ok) throw new Error('Failed to fetch project images')
    
    const images: ApiImage[] = await response.json()
    if (images.length > 0) {
      const imageUrl = await getProjectImage(projectId, images[0].id)
      return imageUrl
    }
  } catch (error) {
    console.error('Error fetching project images:', error)
  }
  return undefined
}

async function loadProjects() {
  try {
    const response = await authFetch(API_BASE + '/projects', {
      method: 'GET',
      credentials: 'include',
    })
    
    if (!response.ok) throw new Error(`Failed to fetch projects: ${response.status}`)
    
    const data: ApiProject[] = await response.json()

    // Create base projects first
    projects.value = data.map((apiProj) => ({
      id: apiProj.id,
      name: apiProj.name,
      lastEdited: daysSince(apiProj.updated_at),
      imageUrl: '/placeholder.svg?height=400&width=600',
    }))

    // Then fetch images for each project
    for (const project of projects.value) {
      const imageUrl = await getProjectImages(project.id)
      if (imageUrl) {
        project.imageUrl = imageUrl
      }
    }
  } catch (error) {
    console.error('Error loading projects:', error)
  }
}

async function createNewProject() {
  const name = prompt('Please enter the new project name:')
  if (!name) return

  try {
    const response = await authFetch(API_BASE + '/projects', {
      method: 'POST',
      credentials: 'include',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ name }),
    })

    if (!response.ok) throw new Error(`Failed to create project: ${response.statusText}`)

    const data = await response.json()
    projects.value.push({
      id: data.id,
      name: data.name,
      imageUrl: '/placeholder.svg?height=400&width=600',
      lastEdited: 0,
    })
  } catch (error) {
    console.error('Error creating project:', error)
  }
}

async function deleteProject(projectId: string) {
  try {
    const response = await authFetch(API_BASE + `/projects/${projectId}`, {
      method: 'DELETE',
      credentials: 'include',
    })

    if (!response.ok) throw new Error(`Failed to delete project: ${response.status}`)

    projects.value = projects.value.filter(p => p.id !== projectId)
  } catch (error) {
    console.error('Error deleting project:', error)
  }
}

const filteredProjects = computed(() => {
  if (!searchQuery.value) return projects.value
  const query = searchQuery.value.toLowerCase()
  return projects.value.filter((p) => p.name.toLowerCase().includes(query))
})

async function uploadZip(projectId: string) {
  try {
    // Step 1: Prompt user to select a zip file
    const input = document.createElement('input')
    input.type = 'file'
    input.accept = '.zip'

    return new Promise<void>((resolve, reject) => {
      input.onchange = async () => {
        const file = input.files?.[0]
        if (!file) return reject('No file selected')

        const formData = new FormData()
        formData.append('File', file) // Match the server's expected key name

        // Step 2: Make the POST request
        try {
          const response = await authFetch(`${API_BASE}/projects/${projectId}/images`, {
            method: 'POST',
            credentials: 'include',
            body: formData,
          })

          if (!response.ok) {
            alert(`Failed to upload zip file: ${response.statusText}`)
          }

          // Step 3: Success handling
          alert('Zip file uploaded successfully!')
          resolve()
        } catch (error) {
          console.error('Error uploading zip file:', error)
          alert('Error uploading zip file')
          reject(error)
        }
      }

      input.click()
    })
  } catch (error) {
    console.error('Error in uploadZip:', error)
  }
}

onMounted(() => {
  loadProjects()
})
</script>

<template>
  <div class="min-h-screen bg-[#030712] flex">
    <!-- Sidebar -->
    <div class="w-64 bg-[#0f1629] p-6 flex flex-col">
      <!-- User Info -->
      <div class="flex items-center space-x-3 mb-8">
        <Avatar class="h-10 w-10 bg-white">
          <AvatarImage src="/placeholder.svg" />
          <AvatarFallback class="bg-white text-[#030712]">
            {{ userName?.charAt(0) ?? 'U' }}
          </AvatarFallback>
        </Avatar>
        <div class="flex flex-col">
          <span class="text-white">{{ userName }}</span>
          <span class="text-sm text-[#969696]">{{ userEmail }}</span>
        </div>
      </div>

      <!-- Projects List -->
      <div class="flex-1">
        <h2 class="text-white font-medium mb-4">Projects</h2>
        <div class="space-y-1">
          <Button
            v-for="project in projects"
            :key="project.id"
            variant="ghost"
            class="w-full justify-start text-[#969696] hover:text-white hover:bg-[#6D28D9]/20"
            @click="router.push(`/project/${project.id}`)"
          >
            {{ project.name }}
          </Button>
        </div>
      </div>

      <!-- Logo and Actions -->
      <div class="space-y-3 mt-auto">
        <h1 class="text-2xl text-center font-bold bg-gradient-to-r from-[#6D28D9] to-white bg-clip-text text-transparent tracking-tight">
          PICTURAS
        </h1>
        <Button class="w-full bg-[#6D28D9] hover:bg-[#5b21b6] text-white" @click="createNewProject">
          New Project
        </Button>
        <Button
          class="w-full bg-[#DD3592] hover:bg-[#c42e81] text-white"
          @click="router.push('/subscriptions')"
        >
          Upgrade Subscription
        </Button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 p-8">
      <!-- Search -->
      <div class="max-w-4xl mx-auto mb-8">
        <div class="relative">
          <SearchIcon class="absolute left-4 top-1/2 -translate-y-1/2 text-[#969696] h-5 w-5" />
          <Input
            v-model="searchQuery"
            placeholder="Search for projects"
            class="w-full pl-12 py-6 bg-white border-0 text-[#030712] placeholder:text-[#969696] focus-visible:ring-[#6D28D9] rounded-full"
          />
        </div>
      </div>

      <!-- Projects Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 max-w-6xl mx-auto">
        <!-- Project Cards -->
        <Card
          v-for="project in filteredProjects"
          :key="project.id"
          class="group bg-[#0f1629] overflow-hidden transition-all duration-300 hover:scale-105 hover:shadow-xl hover:shadow-[#6D28D9]/10"
        >
          <div 
            class="aspect-video relative cursor-pointer"
            @click="router.push(`/project/${project.id}`)"
          >
            <img 
              :src="project.imageUrl" 
              :alt="project.name" 
              class="w-full h-full object-cover"
            />
          </div>

          <CardFooter class="p-4 bg-[#0f1629] flex items-center justify-between">
            <div>
              <h3 class="text-white font-medium">{{ project.name }}</h3>
              <p class="text-sm text-[#969696]">
                Last edited - {{ project.lastEdited }} days ago
              </p>
            </div>
            <div class="flex gap-2">
              <Button
                variant="ghost"
                size="icon"
                class="text-[#969696] hover:text-white hover:bg-[#6D28D9]/20"
                @click="router.push(`/project/${project.id}`)"
              >
                <PencilIcon class="h-4 w-4" />
              </Button>
              <Button
                variant="ghost"
                size="icon"
                class="text-red-500 hover:text-red-400 hover:bg-red-500/20"
                @click="deleteProject(project.id)"
              >
              <Trash2 class="h-4 w-4" />
              </Button>
              <Button
                variant="ghost"
                size="icon"
                class="text-[#969696] hover:text-white hover:bg-[#6D28D9]/20"
                @click="uploadZip(project.id)"
              >
                <ImageUp class="h-4 w-4" />
              </Button>
            </div>
          </CardFooter>
        </Card>

        <!-- Create New Project Card -->
        <Card
          class="group bg-[#0f1629] overflow-hidden transition-all duration-300 hover:scale-105 hover:shadow-xl hover:shadow-[#6D28D9]/10 cursor-pointer"
          @click="createNewProject"
        >
          <div class="aspect-video relative flex items-center justify-center bg-[#0f1629]/50">
            <PlusIcon class="h-16 w-16 text-[#6D28D9]" />
          </div>
          <CardFooter class="p-4 bg-[#0f1629] flex items-center justify-center">
            <h3 class="text-white font-medium">Create New Project</h3>
          </CardFooter>
        </Card>
      </div>
    </div>
  </div>
</template>