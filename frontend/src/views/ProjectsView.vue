<template>
    <div class="min-h-screen bg-[#030712] flex">
      <!-- Sidebar -->
      <div class="w-64 bg-[#0f1629] p-6 flex flex-col">
        <!-- User Info -->
        <div v-if="user.loggedIn()" class="flex items-center space-x-3 mb-8">
          <Avatar class="h-10 w-10 bg-white">
            <AvatarImage src="/placeholder.svg" />
            <AvatarFallback class="bg-white text-[#030712]">JD</AvatarFallback>
          </Avatar>
          <div class="flex flex-col">
            <span class="text-white">{{user.name}}</span>
            <span class="text-sm text-[#969696]">{{user.email}}</span>
          </div>
        </div>
        <div v-else>
          <Button class="w-full px-4 py-2 mb-10 text-white rounded-full" @click="router.push('/register')">
            Sign In
          </Button>
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
              :class="{ 'bg-[#6D28D9]/20 text-white': selectedProject?.id === project.id }"
              @click="selectProject(project)"
            >
              {{ project.name }}
            </Button>
          </div>
        </div>
  
        <!-- Logo and Actions -->
        <div class="space-y-3 mt-auto">
          <h1 class="text-2xl text-center font-bold bg-gradient-to-r from-[#6D28D9] to-white bg-clip-text text-transparent tracking-tight"
                >PICTURAS
        </h1>
          <Button class="w-full rounded-full bg-[#6D28D9] hover:bg-[#5b21b6] text-white">
            New Project
          </Button>
          <Button 
            class="w-full rounded-full bg-[#DD3592] hover:bg-[#c42e81] text-white"
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
  
        <!-- Drag & Drop Area -->
        <div
          class="max-w-4xl mx-auto mb-8"
          @dragenter.prevent="dragActive = true"
          @dragleave.prevent="dragActive = false"
          @dragover.prevent
          @drop.prevent="handleDrop"
        >
          <div
            class="border-2 border-dashed border-gray-700 bg-[#0f1629] rounded-lg p-12 text-center transition-all"
            :class="{
              'border-[#6D28D9] bg-[#6D28D9]/5': dragActive,
              'hover:border-[#6D28D9] hover:bg-[#6D28D9]/5': !dragActive
            }"
          >
            <UploadIcon class="h-12 w-12 mx-auto mb-4 text-[#969696]" />
            <p class="text-[#969696] text-lg">
              Drag and drop an image to create a new project
            </p>
          </div>
        </div>
  
        <!-- Projects Grid -->
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 max-w-6xl mx-auto">
          <div
            v-for="project in filteredProjects"
            :key="project.id"
            class="group bg-[#0f1629] rounded-lg overflow-hidden transition-all duration-300 hover:scale-105 hover:shadow-xl hover:shadow-[#6D28D9]/10"
          >
            <div class="aspect-video relative">
              <img
                :src="project.image"
                :alt="project.name"
                class="w-full h-full object-cover"
              />
            </div>
            
            <div class="p-4 bg-[#0f1629] flex items-center justify-between">
              <div>
                <h3 class="text-white font-medium">{{ project.name }}</h3>
                <p class="text-sm text-[#969696]">
                  Last edited - {{ project.lastEdited }} days ago
                </p>
              </div>
              <Button 
                variant="ghost" 
                size="icon"
                class="text-[#969696] hover:text-white hover:bg-[#6D28D9]/20"
              >
                <PencilIcon class="h-4 w-4" />
              </Button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, computed, onMounted } from 'vue'
  import { useRouter } from 'vue-router'
  import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar'
  import { Button } from '@/components/ui/button'
  import { Input } from '@/components/ui/input'
  import { SearchIcon, UploadIcon, PencilIcon } from 'lucide-vue-next'
  import  { useUserStore } from '@/stores/user'
  
  interface Project {
    id: number
    name: string
    image: string
    lastEdited: number
  }
  
  const user = useUserStore()
  const router = useRouter()
  const searchQuery = ref('')
  const dragActive = ref(false)
  const selectedProject = ref<Project | null>(null)
  
  const projects = ref<Project[]>([
    {
      id: 1,
      name: 'Mountain Landscape',
      image: '/placeholder.svg?height=400&width=600',
      lastEdited: 367
    },
    {
      id: 2,
      name: 'Portrait Session',
      image: '/placeholder.svg?height=400&width=600',
      lastEdited: 368
    },
    {
      id: 3,
      name: 'City Nightscape',
      image: '/placeholder.svg?height=400&width=600',
      lastEdited: 369
    }
  ])
  
  const filteredProjects = computed(() => {
    let filtered = projects.value
    
    if (selectedProject.value) {
      filtered = filtered.filter(p => p.id === selectedProject.value?.id)
    } else if (searchQuery.value) {
      const query = searchQuery.value.toLowerCase()
      filtered = filtered.filter(p => p.name.toLowerCase().includes(query))
    }
    
    return filtered
  })
  
  const selectProject = (project: Project) => {
    selectedProject.value = selectedProject.value?.id === project.id ? null : project
  }
  
  const getProjects = () => {
    // TODO pedido a API para obter os projetos do usuário
    // não sei sei é preciso criar uma storage para guardar os projetos, por causa de haver usuarios sem conta

    if (useUserStore().loggedIn()) {
      // TODO alterar para usar api de dados
      /*
      const response = await fetch(`https://PedidoAPIProjetos/${useUserStore().uuid}`)
      const data = await response.json()
      console.log(data)
      */
     console.log('Está logado')
    }
    else
    {
      //const response = await fetch(`https://PedidoAPIProjetos/`)}`)
      
      //projects.value = []
      console.log('Não está logado')
    }
  }

  const handleDrop = (event: DragEvent) => {
    dragActive.value = false
    const files = event.dataTransfer?.files
    if (files && files.length > 0) {
      // Handle file upload and project creation
      console.log('File dropped:', files[0])
    }
  }
  onMounted(() => {
    getProjects()
  })
  </script>
  
  