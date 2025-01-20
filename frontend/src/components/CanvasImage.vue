<template>
    <div
      ref="canvasContainer"
      class="relative border rounded-lg overflow-hidden w-full h-full"
      @mousedown="onMouseDown"
      @mousemove="onMouseMove"
      @mouseup="onMouseUp"
      @mouseleave="onMouseUp"
      @wheel="onWheel"
    >
      <!-- Main container with transform -->
      <div
        class="image-container absolute"
        :style="{
          transform: `scale(${zoom}) translate(${panX}px, ${panY}px)`,
          transformOrigin: '0 0',
        }"
      >
        <!-- Image Wrapper to constrain overlays -->
        <div class="relative">
          <!-- Main Image -->
          <img
            v-if="image"
            :src="image.image"
            class="max-w-full max-h-full"
            @load="onImageLoad"
            alt="Editable image"
          />
  
          <!-- Tool Overlays Container - constrained to image size -->
          <div 
            class="tool-overlays absolute inset-0"
            :style="{
              width: `${imageSize.width}px`,
              height: `${imageSize.height}px`
            }"
          >
            <!-- Crop Overlay -->
            <div 
              v-if="tool === 'crop'" 
              class="absolute inset-0"
              :style="{
                width: `${imageSize.width}px`,
                height: `${imageSize.height}px`
              }"
            >
              <!-- Dark overlay outside crop area -->
                <div
                  class="absolute bg-black/50 border-2 border-primary"
                  :style="{
                    left: `${Math.min(toolValues.startX, toolValues.endX)}px`,
                    top: `${Math.min(toolValues.startY, toolValues.endY)}px`,
                    width: `${Math.abs(toolValues.endX - toolValues.startX)}px`,
                    height: `${Math.abs(toolValues.endY - toolValues.startY)}px`,
                    boxShadow: '256 256 256 9999px rgba(0, 0, 0, 0)'
                  }"
                ></div>
            </div>

            <!-- Scale Preview -->
            <div 
              v-if="tool === 'scale'" 
              class="absolute inset-0"
              :style="{
                width: `${imageSize.width * (toolValues.scaleX / 100)}px`,
                height: `${imageSize.height * (toolValues.scaleY / 100)}px`
              }"
            >
              <!-- Dark overlay outside scaled area -->
              <div class="absolute inset-0 bg-black/50">
                <div
                  class="absolute bg-transparent border-2 border-primary"
                  :style="{
                    left: `${0}px`,
                    top: `${0}px`,
                    width: `${imageSize.width * (toolValues.scaleX / 100)}px`,
                    height: `${imageSize.height * (toolValues.scaleY / 100)}px`,
                    boxShadow: '256 256 256 9999px rgba(0, 0, 0, 0)'
                  }"
                >
                  <!-- Scale dimensions display -->
                  <div class="absolute inset-0 flex items-center justify-center text-white text-sm bg-black/20">
                    {{ Math.round(toolValues.scaleX) }}% x {{ Math.round(toolValues.scaleY) }}%
                  </div>
                </div>
              </div>
            </div>
  
            <!-- Border Preview -->
            <div
              v-if="tool === 'addBorder'"
              class="absolute"
              :style="{
                inset: `${0}px`,
                border: `${toolValues.borderSize}px solid ${toolValues.borderColor}`,
                maxWidth: `${imageSize.width}px`,
                maxHeight: `${imageSize.height}px`
              }"
            ></div>
  
            <!-- Brightness/Contrast Overlay -->
            <div
              v-if="['adjustBrightness', 'adjustContrast'].includes(tool)"
              class="absolute inset-0"
              :style="{
                filter: getFilterStyle(tool),
                pointerEvents: 'none',
                maxWidth: `${imageSize.width}px`,
                maxHeight: `${imageSize.height}px`
              }"
            >
              <div class="absolute bottom-4 right-4 bg-primary text-white px-2 py-1 rounded text-sm">
                {{ tool === 'adjustBrightness' ? 'Brightness' : 'Contrast' }}: 
                {{ Math.round(tool === 'adjustBrightness' ? toolValues.brightness : toolValues.contrast) }}%
              </div>
            </div>
  
            <!-- Rotate Control -->
            <div
              v-if="tool === 'rotate'"
              class="absolute inset-0 flex items-center justify-center"
              :style="{ 
                transform: `rotate(${toolValues.angle}deg)`,
                maxWidth: `${imageSize.width}px`,
                maxHeight: `${imageSize.height}px`
              }"
            >
              <div class="absolute w-full h-full border-2 border-dashed border-primary">
                <!-- Rotation handle -->
                <div
                  class="absolute -top-12 left-1/2 transform -translate-x-1/2"
                  @mousedown.stop="startRotate"
                >
                  <Button
                    variant="outline"
                    size="icon"
                    class="rounded-full bg-white hover:bg-white/90"
                  >
                    <RotateCw class="h-4 w-4" />
                  </Button>
                  <div class="mt-2 bg-primary text-white px-2 py-1 rounded text-sm text-center">
                    {{ Math.round(toolValues.angle) }}°
                  </div>
                </div>
              </div>
            </div>
  
            <!-- Blur Preview -->
            <div
              v-if="tool === 'blur'"
              class="absolute inset-0"
              :style="{
                filter: `blur(${toolValues.blurRadius}px)`,
                pointerEvents: 'none',
                maxWidth: `${imageSize.width}px`,
                maxHeight: `${imageSize.height}px`
              }"
            >
              <div class="absolute bottom-4 right-4 bg-primary text-white px-2 py-1 rounded text-sm">
                Blur: {{ toolValues.blurRadius }}px
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </template>
  
  <!-- Script and Style sections remain unchanged -->
  <script setup>
  import { ref, computed, onUnmounted, watch } from 'vue';
  import { Button } from '@/components/ui/button';
  import { RotateCw } from 'lucide-vue-next';
  
  const props = defineProps({
    tool: {
      type: String,
      required: true
    },
    image: {
      type: String,
      required: true
    },
    zoom: {
      type: Number,
      default: 1
    },
    toolValues: {
      type: Object,
      default: () => ({})
    }
  });
  
  const emit = defineEmits(['update-tool-data', 'apply-tool', 'image-load']);
  
  const canvasContainer = ref(null);
  const panX = ref(0);
  const panY = ref(0);
  const isPanning = ref(false);
  const panStart = ref({ x: 0, y: 0 });
  const imageSize = ref({ width: 0, height: 0 });
  
  const getFilterStyle = (tool) => {
    if (tool === 'adjustBrightness') {
      return `brightness(${1 + props.toolValues.brightness / 100})`;
    }
    if (tool === 'adjustContrast') {
      return `contrast(${1 + props.toolValues.contrast / 100})`;
    }
    return '';
  };
  
  const onMouseDown = (event) => {
    if (props.tool === 'move') {
      isPanning.value = true;
      panStart.value = {
        x: event.clientX - panX.value,
        y: event.clientY - panY.value
      };
    }
  };
  
  const onMouseMove = (event) => {
    if (isPanning.value) {
      panX.value = event.clientX - panStart.value.x;
      panY.value = event.clientY - panStart.value.y;
    }
  };
  
  const onMouseUp = () => {
    isPanning.value = false;
  };
  
  const onWheel = (event) => {
    if (event.ctrlKey || event.metaKey) {
      event.preventDefault();
      const delta = event.deltaY > 0 ? 0.9 : 1.1;
      emit('update-tool-data', { zoom: Math.max(0.1, Math.min(5, props.zoom * delta)) });
    }
  };
  
  const startRotate = (event) => {
    event.stopPropagation();
    const rect = canvasContainer.value.getBoundingClientRect();
    const centerX = rect.left + rect.width / 2;
    const centerY = rect.top + rect.height / 2;
    const startAngle = Math.atan2(event.clientY - centerY, event.clientX - centerX);
    const initialAngle = props.toolValues.angle || 0;
  
    const onMouseMove = (moveEvent) => {
      const currentAngle = Math.atan2(moveEvent.clientY - centerY, moveEvent.clientX - centerX);
      let angleDiff = (currentAngle - startAngle) * (180 / Math.PI);
      let newAngle = initialAngle + angleDiff;
      
      // Normalize angle to range [-180, 180]
      newAngle = ((newAngle + 180) % 360) - 180;
      
      emit('update-tool-data', { angle: newAngle });
    };
  
    const onMouseUp = () => {
      window.removeEventListener('mousemove', onMouseMove);
      window.removeEventListener('mouseup', onMouseUp);
      emit('apply-tool', props.toolValues);
    };
  
    window.addEventListener('mousemove', onMouseMove);
    window.addEventListener('mouseup', onMouseUp);
  };
  
  const onImageLoad = (event) => {
    imageSize.value = {
      width: event.target.naturalWidth,
      height: event.target.naturalHeight
    };
    emit('image-load', imageSize.value);
  };
  
  onUnmounted(() => {
    window.removeEventListener('mousemove', onMouseMove);
    window.removeEventListener('mouseup', onMouseUp);
  });
  </script>
  
  <style scoped>
  .image-container {
    will-change: transform;
    transition: filter 0.2s ease;
  }
  
  .tool-overlays {
    pointer-events: none;
  }
  
  .tool-overlays > * {
    pointer-events: auto;
  }
  
  </style>