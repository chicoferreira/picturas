<template>
  <div class="flex flex-col h-screen"
    @wheel="handleZoom" >

    <AlertDialog :open="!!error">
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Error</AlertDialogTitle>
          <AlertDialogDescription>
            {{ error }}
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogAction @click="error = null">OK</AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>

    <!-- Main Content Area -->
    <div v-if="!loading" class="flex flex-grow min-h-0">
      <!-- Resizable Sidebar using shadcn Separator -->
      <div
        ref="sidebar"
        :style="{ width: sidebarWidth + 'px'}"
        class="bg-background border-r relative h-full"
      >
        <SidebarTool
          :selectedTool="selectedTool"
          :imageSize="imageSize"
          @select-tool="onToolSelect"
          @update-tool-values="updateToolValues"
          @add-to-chain="addToChain"
        />
        <Separator 
          orientation="vertical" 
          ref="resizeHandle" 
          class="cursor-col-resize absolute right-0 top-0 h-full" 
          @mousedown="onMouseDown" 
        />
      </div>

      <!-- Main Content with Operation Chain -->
      <div class="flex-grow flex flex-col">
        <!-- Canvas Area -->
        <div class="flex-grow flex items-center justify-center bg-background relative overflow-hidden">
          <!-- Image Actions Component -->
          <ImageActions
  :max-width="maxWIDTH"
  :max-height="maxHEIGHT"
  :has-images="images.length > 0"
  :images="images"
  :operation-chain="operationChain"
  @image-selected="handleNewImage"
  @remove-image="removeCurrentImage"
/>

          <div v-if="images.length > 1" class="absolute left-4 z-10">
            <Button variant="outline" size="icon" @click="prevImage" @keydown.left="prevImage">
              <ChevronLeft class="h-4 w-4" />
            </Button>
          </div>

          <CanvasImage
            :tool="selectedTool.name"
            :image="currentImage"
            :zoom="zoom"
            :toolValues="selectedTool.values"
            @update-tool-data="updateToolData"
            @image-load="onImageLoad"
          />
          
          <div v-if="images.length > 1" class="absolute right-4 z-10">
            <Button variant="outline" size="icon" @click="nextImage" @keydown.right="nextImage">
              <ChevronRight class="h-4 w-4" />
            </Button>
          </div>

          <!-- No Image State -->
          <div 
            v-if="images.length === 0" 
            class="absolute inset-0 flex items-center justify-center"
          >
            <div class="text-center text-muted-foreground">
              <Upload class="h-12 w-12 mx-auto mb-4" />
              <p>No images added yet</p>
              <p class="text-sm">Click "Add Image" to get started</p>
            </div>
          </div>
          <div v-if="images.length > 1" class="absolute right-4 z-10">
            <Button variant="outline" size="icon" @click="nextImage" @keydown.right="nextImage">
              <ChevronRight class="h-4 w-4" />
            </Button>
          </div>
        </div>

        <!-- Operation Chain Fixed at Bottom -->
        <div class="border-t bg-background p-4">
    <h3 class="text-lg font-semibold mb-2">Operation Chain</h3>
    <ScrollArea class="w-full mb-4" orientation="horizontal">
      <div class="flex space-x-2">
        <div 
          v-for="(op, index) in operationChain" 
          :key="op.id" 
          class="flex-shrink-0 w-[200px] flex flex-col bg-accent/50 p-4 rounded"
        >
          <div class="flex items-center justify-between mb-2">
            <span class="font-medium">{{ formatProcedureName(op.procedure) }}</span>
            <Button variant="ghost" size="sm" @click="removeFromChain(index)">
              <X class="h-4 w-4" />
            </Button>
          </div>
          
          <span class="text-sm text-muted-foreground mb-2">
            {{ formatParameters(op.parameters) }}
          </span>
          
          <!-- Add Preview Button -->
          <Button 
            v-if="op.processedImages?.length"
            variant="secondary" 
            size="sm"
            class="mt-auto"
            @click="showVersionsForOperation(op)"
          >
            <Eye class="h-4 w-4 mr-2" />
            View Results ({{ op.processedImages.length }})
          </Button>
        </div>
      </div>
      <ScrollBar orientation="horizontal" />
    </ScrollArea>
    
    <!-- Operation Actions -->
    <div v-if="operationChain.length > 0" class="flex space-x-2">
      <Button class="flex-1" @click="openApplyDialog">Apply Chain</Button>
      <Button variant="outline" class="flex-1" @click="openClearDialog">Clear Chain</Button>
    </div>
  </div>

  <!-- Image Versions Dialog -->
  <Dialog :open="showVersionDialog" @update:open="showVersionDialog = false">
    <DialogContent class="max-w-3xl max-h-[80vh]">
      <DialogHeader>
        <DialogTitle>Processed Images</DialogTitle>
      </DialogHeader>
      
      <div class="grid grid-cols-2 gap-4 overflow-y-auto p-4">
        <div v-for="version in selectedVersions" :key="version.id" class="relative">
          <img 
            :src="version.url" 
            :alt="`Processed image ${version.id}`"
            class="w-full h-48 object-cover rounded-lg"
          />
          <div class="absolute bottom-0 left-0 right-0 bg-black/50 text-white p-2 text-sm">
            Original Image: {{ 
              images.find(img => img.id === version.original_image_id)?.name || 'Unknown'
            }}
          </div>
        </div>
      </div>
    </DialogContent>
  </Dialog>
      </div>
    </div>

    <!-- Apply Chain Dialog -->
    <AlertDialog :open="showApplyDialog">
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Apply Operation Chain</AlertDialogTitle>
          <AlertDialogDescription>
            Are you sure you want to apply the current operation chain? This action cannot be undone.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel @click="closeApplyDialog">Cancel</AlertDialogCancel>
          <AlertDialogAction @click="confirmApplyChain">Apply</AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>

    <!-- Clear Chain Dialog -->
    <AlertDialog :open="showClearDialog">
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Clear Operation Chain</AlertDialogTitle>
          <AlertDialogDescription>
            Are you sure you want to clear the current operation chain? All operations will be removed.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel @click="closeClearDialog">Cancel</AlertDialogCancel>
          <AlertDialogAction @click="confirmClearChain">Clear</AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue';
import { Button } from '@/components/ui/button';
import { Separator } from '@/components/ui/separator';
import { useRoute } from 'vue-router';
import { ScrollArea, ScrollBar } from '@/components/ui/scroll-area';
import { 
  AlertDialog,
  AlertDialogContent,
  AlertDialogHeader,
  AlertDialogFooter,
  AlertDialogTitle,
  AlertDialogDescription,
  AlertDialogAction,
  AlertDialogCancel,
} from '@/components/ui/alert-dialog';
import {
  DialogHeader,
  DialogContent,
  Dialog,
  DialogTitle
} from '@/components/ui/dialog';
import { ChevronLeft, ChevronRight, X } from 'lucide-vue-next';
import SidebarTool from '../components/SidebarTool.vue';
import CanvasImage from '../components/CanvasImage.vue';
import { Upload } from 'lucide-vue-next';
import ImageActions from '../components/ImageActions.vue';
import { useAuth } from '@/lib/auth'
import axios from 'axios'

const route = useRoute();
const projectId = route.params.id;
const loading = ref(true);
const error = ref(null);
const sidebarWidth = ref(200);
const images = ref([]);
const currentImageIndex = ref(0);
const zoom = ref(1);
const imageSize = ref({ width: 0, height: 0 });
const operationChain = ref([]);
const showApplyDialog = ref(false);
const showClearDialog = ref(false);
const API_BASE = process.env.VITE_API_BASE  || 'http://localhost:80/api/v1';
const endpoints = {
  project: `${API_BASE}/projects/${projectId}`,
  images: `${API_BASE}/projects/${projectId}/images`,
  tools: `${API_BASE}/projects/${projectId}/tools`,
  applyTools: `${API_BASE}/projects/${projectId}/tools/apply`,
  imageVersions: `${API_BASE}/projects/${projectId}/tools/images`
}
const { authFetch } = useAuth()
const maxWIDTH = ref(1920);
const maxHEIGHT = ref(1080); 
let ws = null;
let selectedVersions = ref([]);
const openApplyDialog = () => {
  showApplyDialog.value = true;
};

const closeApplyDialog = () => {
  showApplyDialog.value = false;
};

const openClearDialog = () => {
  showClearDialog.value = true;
};

const closeClearDialog = () => {
  showClearDialog.value = false;
};

const handleNewImage = async (imageData) => {
  const formData  = new FormData();
  formData.append(`File`, imageData.file);
  console.log("Posting image");
  let response = await authFetch(endpoints.images, {
    method: "POST",
    credentials: 'include',
    body: formData
  })
  response = await response.json();
  console.log("images.value " + images.value);
  images.value.push({
    id: response[0].id,
    data: imageData.data});
  currentImageIndex.value = images.value.length - 1;
};


const selectedTool = ref({
  name: 'move',
  label: 'Move',
  values: {}
});

const currentImage = computed(() => images.value[currentImageIndex.value] ? images.value[currentImageIndex.value].data : []);

const onToolSelect = (tool) => {
  selectedTool.value = {
    name: tool.name,
    label: tool.label,
    values: getDefaultValuesForTool(tool.name)
  };
};

const updateToolValues = (values) => {
  selectedTool.value = {
    ...selectedTool.value,
    values
  };
};

const updateToolData = (data) => {
  updateToolValues({ ...selectedTool.value.values, ...data });
};

const addToChain =async (data) => {
  const operation = convertToBackendFormat(selectedTool.value.name, data);
  if (operation) {
    const [toolName, Nparameters] = Object.entries(operation)[0];
    let tempChain = operationChain.value;
    tempChain.push({
      procedure: toolName,
      parameters: Nparameters
    })
    let response = await authFetch(endpoints.tools, {
      method: 'PUT',
      credentials: 'include',
      headers: {
          'Content-Type': 'application/json'
      },
      body:JSON.stringify(tempChain)});

    response = await response.json();
    operationChain.value = response;
  }
};

const removeFromChain = async (index) => {
  operationChain.value.splice(index, 1);
  let response = await authFetch(endpoints.tools, {
      method: 'PUT',
      credentials: 'include',
      headers: {
          'Content-Type': 'application/json'
      },
      body:JSON.stringify(operationChain.value)});

    response = await response.json();
    operationChain.value = response;
};

const confirmClearChain = async () => {
  operationChain.value = [];
  let response = await authFetch(endpoints.tools, {
      method: 'PUT',
      credentials: 'include',
      headers: {
          'Content-Type': 'application/json'
      },
      body:JSON.stringify(operationChain.value)});

    response = await response.json();
    operationChain.value = response;
  closeClearDialog();
};

const confirmApplyChain = async () => {
  try {
    let imageIds = [];
    for (let i = 0; i < images.value.length; i++) {
      imageIds.push(images.value[i].id);
    }
    console.log('Applying image:', imageIds);
    let response = await authFetch(endpoints.applyTools, {
      method: 'POST',
      credentials: 'include', 
      headers: {
          'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        "filter_images":imageIds
      })
    })
    closeApplyDialog();
    response = await response.json();
    console.log("Response" + JSON.stringify(response));
  } catch (error) {
    console.error('Error applying operation chain:', error);
  }
};

const formatProcedureName = (procedure) => {
  return procedure.replace(/([A-Z])/g, ' $1').trim();
};

const formatParameters = (params) => {
  if (!params) return '';
  return Object.entries(params)
    .map(([key, value]) => {
      if (Array.isArray(value)) {
        return `${key}: (${value.join(', ')})`;
      }
      return `${key}: ${value}`;
    })
    .join(', ');
};

const convertToBackendFormat = (toolName, data) => {
  // Create clean copies of the data to prevent circular references
  const cleanData = JSON.parse(JSON.stringify(data));
  
  switch (toolName) {
    case 'crop':
      return {
        Crop: {
          start: [
            Math.min(Math.round(cleanData.startX), Math.round(cleanData.endX)),
            Math.min(Math.round(cleanData.startY), Math.round(cleanData.endY))
          ],
          end: [
            Math.max(Math.round(cleanData.startX), Math.round(cleanData.endX)),
            Math.max(Math.round(cleanData.startY), Math.round(cleanData.endY))
          ]
        }
      };
    case 'scale':
      return {
        Scale: {
          x: Math.round(cleanData.scaleX),
          y: Math.round(cleanData.scaleY)
        }
      };
    case 'addBorder':
      return {
        AddBorder: {
          size: Math.round(cleanData.borderSize),
          color: hexToRgb(cleanData.borderColor)
        }
      };
    case 'adjustBrightness':
      return {
        AdjustBrightness: {
          value: Math.round(cleanData.brightness)
        }
      };
    case 'adjustContrast':
      return {
        AdjustContrast: {
          value: parseFloat(cleanData.contrast)
        }
      };
    case 'rotate':
      return {
        Rotate: {
          angle: parseFloat(cleanData.angle)
        }
      };
    case 'blur':
      return {
        Blur: {
          radius: Math.round(cleanData.blurRadius)
        }
      };
    case 'watermark':
      return { AddWatermark: null };
    case 'bgRemover':
      return { BgRemover: null };
    case 'grayscale':
      return { Grayscale: null };
    case 'binarization':
      return { Binarization: null };
    default:
      return null;
  }
};
const hexToRgb = (hex) => {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result ? [
    parseInt(result[1], 16),
    parseInt(result[2], 16),
    parseInt(result[3], 16)
  ] : [0, 0, 0];
};

const getDefaultValuesForTool = (toolName) => {
  const maxBorderSize = Math.min(imageSize.value.width || 100, imageSize.value.height || 100) / 4;
  
  switch (toolName) {
    case 'move':
      return {};
    case 'crop':
      return {
        startX: 0,
        startY: 0,
        endX: imageSize.value.width || 100,
        endY: imageSize.value.height || 100
      };
    case 'scale':
      return { scaleX: 100, scaleY: 100 };
    case 'addBorder':
      return {
        borderSize: Math.min(10, maxBorderSize),
        borderColor: '#000000'
      };
    case 'adjustBrightness':
      return { brightness: 0 };
    case 'adjustContrast':
      return { contrast: 0 };
    case 'rotate':
      return { angle: 0 };
    case 'blur':
      return { blurRadius: 0 };
    default:
      return {};
  }
};

// Sidebar Resize Handling
const onMouseDown = (event) => {
  const initialWidth = sidebarWidth.value;
  const startX = event.clientX;

  const onMouseMove = (moveEvent) => {
    const deltaX = moveEvent.clientX - startX;
    sidebarWidth.value = Math.max(initialWidth + deltaX, 200);
  };

  const onMouseUp = () => {
    window.removeEventListener('mousemove', onMouseMove);
    window.removeEventListener('mouseup', onMouseUp);
  };

  window.addEventListener('mousemove', onMouseMove);
  window.addEventListener('mouseup', onMouseUp);
};

const handleZoom = (event) => {
  event.preventDefault(); 
  const zoomStep = 0.05; 
  if (event.deltaY < 0) {
    zoom.value = Math.min(zoom.value + zoomStep, 5); 
  } else {
    zoom.value = Math.max(zoom.value - zoomStep, 0.1);
  }
};

const onImageLoad = (size) => {
  imageSize.value = size;
  if (selectedTool.value.name === 'crop') {
    updateToolValues({
      startX: 0,
      startY: 0,
      endX: size.width,
      endY: size.height
    });
  }
};

const removeCurrentImage = async () => {
  if (images.value.length > 0) {
    await authFetch(endpoints.images + "/" + images.value[currentImageIndex.value].id, {
      method: "DELETE",
      credentials: `include`
    })
    images.value.splice(currentImageIndex.value, 1);
    if (currentImageIndex.value >= images.value.length) {
      currentImageIndex.value = Math.max(0, images.value.length - 1);
    }
  }
};

const nextImage = () => {
  if (currentImageIndex.value < images.value.length - 1) {
    currentImageIndex.value++;
  }else {
    currentImageIndex.value = 0;
  }
};

const prevImage = () => {
  if (currentImageIndex.value > 0) {
    currentImageIndex.value--;
  }else {
    currentImageIndex.value = images.value.length - 1;
  }
};

const connectWebSocket = () => {
  ws = new WebSocket(endpoints.project + '/ws');

  ws.onmessage = (event) => {
    console.log("Received message: " + event)
    const message = JSON.parse(event.data);

    if (operationChain.value.length > 0 && message.tool_id === operationChain.value[operationChain.value.length - 1].toolId) {
      showImagePreview(message.url);
    }
  };

  ws.onclose = () => {
    setTimeout(connectWebSocket, 5000);
  };
};

const showImagePreview = (imageUrl) => {
  const previewArea = document.createElement('div');
  previewArea.style.position = 'fixed';
  previewArea.style.top = '0';
  previewArea.style.left = '0';
  previewArea.style.width = '100vw';
  previewArea.style.height = '100vh';
  previewArea.style.backgroundColor = 'rgba(0, 0, 0, 0.8)';
  previewArea.style.display = 'flex';
  previewArea.style.alignItems = 'center';
  previewArea.style.justifyContent = 'center';
  previewArea.style.zIndex = '9999';

  const image = document.createElement('img');
  image.src = imageUrl;
  image.style.maxWidth = '90%';
  image.style.maxHeight = '90%';
  previewArea.appendChild(image);

  previewArea.addEventListener('click', () => {
    document.body.removeChild(previewArea);
  });

  document.body.appendChild(previewArea);
};

onMounted(async () => {
  try {
    let [imagesResponse, toolsResponse] = await Promise.all([
      authFetch(endpoints.images, {
        method: 'GET',
        credentials: 'include'
      }),
      authFetch(endpoints.tools, {
        method: 'GET',
        credentials: 'include'
      })
    ]);

    // Parse JSON responses
    const imagesData = await imagesResponse.json();
    const toolsData = await toolsResponse.json();

    // Update reactive variables
    images.value = await Promise.all(imagesData.map(async (image) => {
      const response = await authFetch(image.url, {
        method: 'GET',
        credentials: 'include'
      });

      const blob = await response.blob();
      image.data = await convertBlobToBase64(blob);  // Fixed FileReader usage
      return image;
    }));
    console.log("IMAGES DATA " + JSON.stringify(imagesData));
    operationChain.value = toolsData;
  } catch (error) {
    console.error("Error during onMounted:", error.message);
  } finally {
    loading.value = false;
  }
});

// Utility function to convert Blob to Base64
function convertBlobToBase64(blob) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onloadend = () => resolve(reader.result);  // reader.result contains the Base64 string
    reader.onerror = reject;
    reader.readAsDataURL(blob);  // This converts the blob to a base64 string
  });
}

onMounted(() => {
  connectWebSocket();
  window.addEventListener('keydown', (event) => {
    if (images.value.length > 1) {
      if (event.key === 'ArrowRight') {
        nextImage();
      } else if (event.key === 'ArrowLeft') {
        prevImage();
      }
    }
  });
});
</script>

<style scoped>
.image-container {
  will-change: transform;
  transition: filter 0.2s ease;
}
</style>