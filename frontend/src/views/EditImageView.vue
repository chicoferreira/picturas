<script setup lang="ts">

import { Badge } from '@/components/ui/badge'
import {
  Sheet,
  SheetClose,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from '@/components/ui/sheet'
import { Button } from '@/components/ui/button'
import { Drawer, DrawerContent, DrawerDescription, DrawerHeader, DrawerTitle, DrawerTrigger } from '@/components/ui/drawer'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { Textarea } from '@/components/ui/textarea'
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip'
import { Bird, Book, Bot, Code2, CornerDownLeft, LifeBuoy, Mic, Paperclip, Rabbit, Settings, Settings2, Share, SkipBackIcon, SquareTerminal, SquareUser, Triangle, Turtle, Undo2Icon, Trash2Icon } from 'lucide-vue-next'
import { useRouter } from 'vue-router'

import { ref, onMounted } from 'vue'
  interface Tool {
    id: number
    name: string
    icon: string
    type : string[]
    premium: boolean
  }

  const props = defineProps<{
    idProject: string,
    idImage: string
  }>()
  
  const tools = ref<Tool[]>([])
  const toolsType = ref<string[]>([])
  const selectedType = ref('All')

  const imgData = ref({
    id: 1,
    name: 'Uploaded Image',
    image: '/placeholder.svg?height=400&width=600',
  })
  const toolsQueue = ref<Tool[]>([])
  
  const removeFromQueue = (index: number) => {
    toolsQueue.value.splice(index, 1)
  }

  const addToQueue = (tool: Tool) => {
    toolsQueue.value.push(tool)
    }
    

    const getImgData = async () => {
        console.log('Carregando imagem...')
        console.log('id do projeto:', props.idProject)

        // TODO alterar para obter dados da imagem atraveés do id da imagem e id do projeto
        try {
        imgData.value = {
            id: 1,
            name: 'Ramalho',
            image: '/public/ramalho.jpg',
        }
        } catch (error) {
        console.error('Erro ao carregar imagem:', error)
        }
    }
  const getTools = async () => {
    // TODO alterar para obter lista de ferramentas disponíveis, e os tipos existentes
    try {
      tools.value = [
        { id: 1, name: 'Remove Watermark', icon: '✂️', type: ['AI'], premium: true,  },
        { id: 2, name: 'Mudar Cor', icon: '🎨', type: ['Colour'], premium: false },
        { id: 3, name: 'Recorte', icon: '🖌️', type: ['Dimension'], premium: false },
      ]
        toolsType.value = ['AI', 'Colour', 'Dimension']
    } catch (error) {
      console.error('Erro ao carregar ferramentas:', error)
    }
  }
  const getToolsQueue = async () => {
    // TODO alterar sintax para buscar lista de tools em fila de espera na imagem
    toolsQueue.value = [
        { id: 1, name: 'Remove Watermark', icon: '✂️', type: ['AI'], premium: true,  },
        { id: 2, name: 'Mudar Cor', icon: '🎨', type: ['Colour'], premium: false },
        { id: 3, name: 'Recorte', icon: '🖌️', type: ['Dimension'], premium: false },
      ]
  }

  const submit = () => {
    // TODO alterar para processar pedido de edição de imagem
    console.log('Ferramentas selecionadas:', toolsQueue)
    toolsQueue.value = []
    // talvez adicionar um pop up de sucesso
  }

    const filterType = (type: string) => {
        selectedType.value = type
        console.log('Filtrar por:', selectedType.value)
    }

    const router = useRouter()
    const goBack = () => {
        router.back()
        //router.push('/projects/' + props.idProject)
    }
  onMounted(() => {
    getImgData()
    getTools()
    getToolsQueue()
  })
</script>

<template>

  <div class="grid h-screen w-full">
<!--
    <aside class="inset-y fixed  left-0 z-20 flex h-full flex-col border-r">
      <div class="border-b p-2">
        <Button variant="outline" size="icon" aria-label="Home">
          <Triangle class="size-5 fill-foreground" />
        </Button>
      </div>
      <nav class="grid gap-1 p-2">
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger as-child>
              <Button
                variant="ghost"
                size="icon"
                class="rounded-lg bg-muted"
                aria-label="Playground"
              >
                <SquareTerminal class="size-5" />
              </Button>
            </TooltipTrigger>
            <TooltipContent side="right" :side-offset="5">
              Playground
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger as-child>
              <Button
                variant="ghost"
                size="icon"
                class="rounded-lg"
                aria-label="Models"
              >
                <Bot class="size-5" />
              </Button>
            </TooltipTrigger>
            <TooltipContent side="right" :side-offset="5">
              Models
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger as-child>
              <Button
                variant="ghost"
                size="icon"
                class="rounded-lg"
                aria-label="API"
              >
                <Code2 class="size-5" />
              </Button>
            </TooltipTrigger>
            <TooltipContent side="right" :side-offset="5">
              API
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger as-child>
              <Button
                variant="ghost"
                size="icon"
                class="rounded-lg"
                aria-label="Documentation"
              >
                <Book class="size-5" />
              </Button>
            </TooltipTrigger>
            <TooltipContent side="right" :side-offset="5">
              Documentation
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger as-child>
              <Button
                variant="ghost"
                size="icon"
                class="rounded-lg"
                aria-label="Settings"
              >
                <Settings2 class="size-5" />
              </Button>
            </TooltipTrigger>
            <TooltipContent side="right" :side-offset="5">
              Settings
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </nav>
      <nav class="mt-auto grid gap-1 p-2">
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger as-child>
              <Button
                variant="ghost"
                size="icon"
                class="mt-auto rounded-lg"
                aria-label="Help"
              >
                <LifeBuoy class="size-5" />
              </Button>
            </TooltipTrigger>
            <TooltipContent side="right" :side-offset="5">
              Help
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger as-child>
              <Button
                variant="ghost"
                size="icon"
                class="mt-auto rounded-lg"
                aria-label="Account"
              >
                <SquareUser class="size-5" />
              </Button>
            </TooltipTrigger>
            <TooltipContent side="right" :side-offset="5">
              Account
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </nav>
    </aside>-->
    <div class="flex flex-col">
      <header class="sticky top-0 z-10 flex h-[57px] items-center gap-1 border-b bg-background px-4">
        <div class = "flex items-center gap-2">
        <Button 
            className="p-0 bg-transparent border-none text-gray-700 hover:text-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 rounded-md transition-colors"
            aria-label="Undo"
            @click="goBack">
            <Undo2Icon/>
        </Button>
        <h1 class="text-xl font-semibold">
          Image {{imgData.id}}
        </h1>
        </div>
        <Drawer>
          <DrawerTrigger as-child>
            <Button variant="ghost" size="icon" class="md:hidden">
              <Settings class="size-4" />
              <span class="sr-only">Settings</span>
            </Button>
          </DrawerTrigger>
          <DrawerContent class="max-h-[80vh]">
            <DrawerHeader>
              <DrawerTitle>Configuration</DrawerTitle>
              <DrawerDescription>
                Configure the settings for the model and messages.
              </DrawerDescription>
            </DrawerHeader>
            <form class="grid w-full items-start gap-6 overflow-auto p-4 pt-0">
              <fieldset class="grid gap-6 rounded-lg border p-4">
                <legend class="-ml-1 px-1 text-sm font-medium">
                  Settings
                </legend>
                <div class="grid gap-3">
                  <Label for="model">Model</Label>
                  <Select>
                    <SelectTrigger
                      id="model"
                      class="items-start [&_[data-description]]:hidden"
                    >
                      <SelectValue placeholder="Select a model" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="genesis">
                        <div class="flex items-start gap-3 text-muted-foreground">
                          <Rabbit class="size-5" />
                          <div class="grid gap-0.5">
                            <p>
                              Neural
                              <span class="font-medium text-foreground">
                                Genesis
                              </span>
                            </p>
                            <p class="text-xs" data-description>
                              Our fastest model for general use cases.
                            </p>
                          </div>
                        </div>
                      </SelectItem>
                      <SelectItem value="explorer">
                        <div class="flex items-start gap-3 text-muted-foreground">
                          <Bird class="size-5" />
                          <div class="grid gap-0.5">
                            <p>
                              Neural
                              <span class="font-medium text-foreground">
                                Explorer
                              </span>
                            </p>
                            <p class="text-xs" data-description>
                              Performance and speed for efficiency.
                            </p>
                          </div>
                        </div>
                      </SelectItem>
                      <SelectItem value="quantum">
                        <div class="flex items-start gap-3 text-muted-foreground">
                          <Turtle class="size-5" />
                          <div class="grid gap-0.5">
                            <p>
                              Neural
                              <span class="font-medium text-foreground">
                                Quantum
                              </span>
                            </p>
                            <p class="text-xs" data-description>
                              The most powerful model for complex
                              computations.
                            </p>
                          </div>
                        </div>
                      </SelectItem>
                    </SelectContent>
                  </Select>
                </div>
                <div class="grid gap-3">
                  <Label for="temperature">Temperature</Label>
                  <Input id="temperature" type="number" placeholder="0.4" />
                </div>
                <div class="grid gap-3">
                  <Label for="top-p">Top P</Label>
                  <Input id="top-p" type="number" placeholder="0.7" />
                </div>
                <div class="grid gap-3">
                  <Label for="top-k">Top K</Label>
                  <Input id="top-k" type="number" placeholder="0.0" />
                </div>
              </fieldset>
              <fieldset class="grid gap-6 rounded-lg border p-4">
                <legend class="-ml-1 px-1 text-sm font-medium">
                  Messages
                </legend>
                <div class="grid gap-3">
                  <Label for="role">Role</Label>
                  <Select default-value="system">
                    <SelectTrigger>
                      <SelectValue placeholder="Select a role" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="system">
                        System
                      </SelectItem>
                      <SelectItem value="user">
                        User
                      </SelectItem>
                      <SelectItem value="assistant">
                        Assistant
                      </SelectItem>
                    </SelectContent>
                  </Select>
                </div>
                <div class="grid gap-3">
                  <Label for="content">Content</Label>
                  <Textarea id="content" placeholder="You are a..." />
                </div>
              </fieldset>
            </form>
          </DrawerContent>
        </Drawer>
      </header>
      <main class="grid flex-1 gap-4 overflow-auto p-4 md:grid-cols-2 lg:grid-cols-3">
        <div class="flex-col relative hidden py-9 px-2 items-start gap-8 md:flex">
        <div class ="flex gap-4">
        <button v-for="type in ['All', ...toolsType]" 
        :key="type" @click="filterType(type)" class="flex items-center gap-2 text-gray-700 hover:text-blue-500 focus:outline-none focus:border-b-2 focus:border-blue-500 rounded-md transition-colors" >
        {{type}}
        </button>
        </div>
        <div class ="flex gap-4">
    <Sheet v-for="tool in tools.filter(tool => selectedType === 'All' || tool.type.includes(selectedType))"
           :key="tool.id">
    <SheetTrigger class="h-22" as-child>
        <Button 
  variant="outline" 
  class="flex flex-col items-center max-w-[80px] border border-gray-300 px-4 py-2 rounded-md text-gray-700 hover:bg-gray-100"
>
  <span class="mb-2">{{tool.icon}}</span>
  <span class="text-center text-pretty" :class="{'text-lime-500': tool.premium}">
    {{tool.name}}
  </span>
</Button>
    </SheetTrigger>
    <SheetContent>
      <SheetHeader>
        <SheetTitle>Edit profile</SheetTitle>
        <SheetDescription>
          Make changes to your profile here. Click save when you're done.
        </SheetDescription>
      </SheetHeader>
      <div class="grid gap-4 py-4">
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="name" class="text-right">
            Name
          </Label>
          <Input id="name" value="Pedro Duarte" class="col-span-3" />
        </div>
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="username" class="text-right">
            Username
          </Label>
          <Input id="username" value="@peduarte" class="col-span-3" />
        </div>
      </div>
      <SheetFooter>
        <SheetClose as-child>
          <Button type="submit">
            Save changes
          </Button>
        </SheetClose>
      </SheetFooter>
    </SheetContent>
  </Sheet>
  </div>
        </div>
        <div class="relative flex h-full min-h-[50vh] flex-col rounded-xl bg-muted/50 p-2 lg:col-span-2">
            <div class="flex-1 flex items-center justify-center">
                <img :src="imgData.image" alt="Uploaded Image" class="rounded-xl mx-auto" />
            </div>
            <div class ="flex-col gap-2 p-1 m-2">
               <h3 class ="font-semibold mb-4"> Fila de Ferramentas</h3> 
               <span v-if="toolsQueue.length===0">Sem ferramentas na fila por enquanto</span>
                <div v-else class="flex justify-between">
                    <div class ="flex gap-5">
                    <div v-for="(tool,index) in toolsQueue" class="flex items">
                        <span class="w-64px h-64px mb-2">{{tool.icon}}</span>
                        <button @click="removeFromQueue(index)">
                            <Trash2Icon class ="w-12px h-12px"/>
                        </button>
                    </div>
                    </div>
                    <Button @click="submit" class="w-auto bg-[#6D28D9] hover:bg-[#5b21b6] transition-colors">
                        Editar
                    </Button>
                </div>
            </div>
        </div>
      </main>
    </div>
  </div>
</template>
