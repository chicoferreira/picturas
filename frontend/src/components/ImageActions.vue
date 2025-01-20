<template>
    <div class="flex gap-2 absolute top-4 right-4 z-20">
      <input
        type="file"
        ref="fileInput"
        accept="image/*"
        class="hidden"
        @change="handleImageUpload"
      />
      <Button 
  variant="destructive"
  class="flex items-center gap-2"
  @click="confirmDeleteImage"
  :disabled="!hasImages"
>
  <Trash2 class="h-4 w-4" />
  Remove Image
</Button>

<AlertDialog :open="showDeleteConfirm">
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>Remove Image</AlertDialogTitle>
      <AlertDialogDescription>
        Are you sure you want to remove the current image? This action cannot be undone.
      </AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter>
      <AlertDialogCancel @click="closeDeleteDialog">Cancel</AlertDialogCancel>
      <AlertDialogAction @click="confirmDelete">Remove</AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
      
      <!-- Add Image Button -->
      <Button 
        variant="secondary" 
        class="flex items-center gap-2"
        @click="$refs.fileInput.click()"
      >
        <Upload class="h-4 w-4" />
        Add Image
      </Button>
  
      <!-- Download Project Button -->
      <Button 
        variant="default"
        class="flex items-center gap-2"
        @click="downloadProject"
        :disabled="!hasImages"
      >
        <Download class="h-4 w-4" />
        Download Project
      </Button>
  
      <!-- Alert Dialog for Size Error -->
      <AlertDialog :open="showSizeError">
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Image Size Error</AlertDialogTitle>
            <AlertDialogDescription>
              The selected image exceeds the maximum allowed dimensions of {{ maxWidth }}x{{ maxHeight }} pixels.
              Please select a smaller image or upgrade your subscription.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogAction @click="showSizeError = false">OK</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  </template>
  
  <script setup>
  import { ref } from 'vue';
  import { Button } from '@/components/ui/button';
  import { Upload, Download } from 'lucide-vue-next';
  import JSZip from 'jszip';
  import { saveAs } from 'file-saver';
  import {
    AlertDialog,
    AlertDialogContent,
    AlertDialogHeader,
    AlertDialogFooter,
    AlertDialogTitle,
    AlertDialogDescription,
    AlertDialogAction,
    AlertDialogCancel
  } from '@/components/ui/alert-dialog';
import { Trash2 } from 'lucide-vue-next';

const emit = defineEmits(['image-selected', 'download-requested', 'remove-image']);

const showDeleteConfirm = ref(false);

const confirmDeleteImage = () => {
  showDeleteConfirm.value = true;
};

const closeDeleteDialog = () => {
  showDeleteConfirm.value = false;
};

const confirmDelete = () => {
  emit('remove-image');
  closeDeleteDialog();
};
  
const props = defineProps({
  maxWidth: {
    type: Number,
    default: 1920
  },
  maxHeight: {
    type: Number,
    default: 1080
  },
  hasImages: {
    type: Boolean,
    default: false
  },
  images: {
    type: Array,
    default: () => []
  },
  operationChain: {
    type: Array,
    default: () => []
  }
});

const downloadProject = async () => {
  try {
    const zip = new JSZip();
    
    props.images.forEach((imageData, index) => {
      // Skip placeholder images
      if (!imageData.startsWith('data:')) return;
      
      const blob = base64ToBlob(imageData);
      zip.file(`image_${index + 1}.png`, blob);
    });

    const metadata = {
      totalImages: props.images.filter(img => img.startsWith('data:')).length,
      operations: props.operationChain,
      createdAt: new Date().toISOString(),
    };
    
    zip.file('project-metadata.json', JSON.stringify(metadata, null, 2));

    const content = await zip.generateAsync({ type: 'blob' });
    saveAs(content, 'image-project.zip');
  } catch (error) {
    console.error('Error downloading project:', error);
  }
};

  const base64ToBlob = (base64String) => {
  const parts = base64String.split(';base64,');
  const contentType = parts[0].split(':')[1];
  const raw = window.atob(parts[1]);
  const rawLength = raw.length;
  const uInt8Array = new Uint8Array(rawLength);

  for (let i = 0; i < rawLength; ++i) {
    uInt8Array[i] = raw.charCodeAt(i);
  }

  return new Blob([uInt8Array], { type: contentType });
};
    
  const fileInput = ref(null);
  const showSizeError = ref(false);
  
  const handleImageUpload = async (event) => {
    const file = event.target.files[0];
    if (!file) return;
  
    // Create an image object to check dimensions
    const img = new Image();
    const objectUrl = URL.createObjectURL(file);
  
    img.onload = () => {
      URL.revokeObjectURL(objectUrl);
      
      if (img.width > props.maxWidth || img.height > props.maxHeight) {
        showSizeError.value = true;
        event.target.value = ''; // Reset file input
        return;
      }
  
      // Convert to base64 for preview
      const reader = new FileReader();
      reader.onload = (e) => {
        emit('image-selected', {
          data: e.target.result,
          name: file.name,
          width: img.width,
          height: img.height
        });
      };
      reader.readAsDataURL(file);
    };
  
    img.src = objectUrl;
  };
  
  </script>