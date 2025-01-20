<template>
    <aside class="w-full h-full p-4 flex flex-col gap-4">
      <!-- Tools List -->
      <div class="space-y-2">
        <Button
          v-for="tool in tools"
          :key="tool.name"
          variant="ghost"
          :class="{ 'bg-accent': selectedTool.name === tool.name }"
          class="w-full justify-start"
          @click="selectTool(tool)"
        >
          <component :is="tool.icon" class="mr-2 h-4 w-4" />
          {{ tool.label }}
        </Button>
      </div>
  
      <!-- Tool Settings -->
      <div v-if="showSettings" class="space-y-1">
        <Separator />
        <div class="flex items-center justify-between">
          <h3 class="text-lg font-semibold">{{ selectedTool.label }} Settings</h3>
          <Button 
            v-if="hasParameters"
            variant="outline" 
            size="sm" 
            @click="resetToolValues"
          >
            Reset
          </Button>
        </div>
  
        <div v-for="(value, key) in currentToolValues" :key="key" class="space-y-1">
          <Label :for="key">{{ formatLabel(key) }}</Label>
  
          <!-- Color Input with Preview -->
          <div v-if="key.toLowerCase().includes('color')" class="flex gap-2">
            <div
              class="w-8 h-8 rounded border"
              :style="{ backgroundColor: currentToolValues[key] }"
            ></div>
            <Input
              :id="key"
              v-model="currentToolValues[key]"
              type="text"
              class="w-24"
              @input="handleColorInput($event, key)"
            />
            <Input
              :id="`${key}-picker`"
              v-model="currentToolValues[key]"
              type="color"
              class="w-12 h-8 p-0"
              @input="handleColorInput($event, key)"
            />
          </div>
  
          <!-- Numeric Input with Slider -->
          <div v-else-if="typeof value === 'number'" class="space-y-0">
            <div class="flex items-center gap-2">
              <Slider
                :id="key"
                :model-value="[currentToolValues[key]]"
                :min="getMinValue(key)"
                :max="getMaxValue(key)"
                :step="getStep(key)"
                class="flex-1"
                @update:model-value="(vals) => handleNumericInput(key, vals[0])"
              />
              <Input
                :id="`${key}-number`"
                v-model.number="currentToolValues[key]"
                type="number"
                :min="getMinValue(key)"
                :max="getMaxValue(key)"
                :step="getStep(key)"
                class="w-20"
                @input="(e) => handleNumericInput(key, parseFloat(e.target.value))"
              />
            </div>
            <div class="flex justify-between text-xs text-muted-foreground">
              <span>{{ getMinValue(key) }}{{ getUnit(key) }}</span>
              <span>{{ getMaxValue(key) }}{{ getUnit(key) }}</span>
            </div>
          </div>
        </div>
  
        <!-- Add to Chain Button -->
        <Button class="w-full mt-4" @click="addToChain">
          Add to Chain
        </Button>
      </div>
    </aside>
  </template>
  
  <script setup>
  import { ref, computed, watch, markRaw  } from 'vue';
  import { Button } from '@/components/ui/button';
  import { Input } from '@/components/ui/input';
  import { Label } from '@/components/ui/label';
  import { Slider } from '@/components/ui/slider';
  import { Separator } from '@/components/ui/separator';
  import {
    Move,
    Crop,
    ZoomIn,
    Square,
    Sun,
    Contrast,
    RotateCw,
    Cloud,
    Type,
    ImageOff,
    Droplet
  } from 'lucide-vue-next';
  import GrayscaleIcon from '@/components/icons/Grayscale.vue'
  import BinarizationIcon from "@/components/icons/Binarization.vue"
  
  const props = defineProps({
    selectedTool: {
      type: Object,
      required: true
    },
    imageSize: {
      type: Object,
      required: true
    }
  });
  
  const emit = defineEmits(['select-tool', 'update-tool-values', 'add-to-chain']);
  
  const tools = ref([
    { name: 'move', label: 'Move', icon: Move },
    { name: 'crop', label: 'Crop', icon: Crop },
    { name: 'scale', label: 'Scale', icon: ZoomIn },
    { name: 'addBorder', label: 'Add Border', icon: Square },
    { name: 'adjustBrightness', label: 'Brightness', icon: Sun },
    { name: 'adjustContrast', label: 'Contrast', icon: Contrast },
    { name: 'rotate', label: 'Rotate', icon: RotateCw },
    { name: 'blur', label: 'Blur', icon: Cloud },
    { name: 'ocr', label: 'OCR', icon: Type },
    {name: 'watermark', label: 'Add Watermark', icon:Droplet},
    {name: 'bgRemover', label: 'Remove Background', icon:ImageOff},
    {name: 'grayscale', label: 'Grayscale', icon:markRaw(GrayscaleIcon)},
    {name: 'binarization', label: 'binarization', icon:markRaw(BinarizationIcon)}
  ]);
  
  const currentToolValues = ref({});
  
  const showSettings = computed(() => {
    // Only hide settings for move tool
    return props.selectedTool.name !== 'move';
  });

  const hasParameters = computed(() => {
    return Object.keys(currentToolValues.value).length > 0;
  });
  
  // Input Handlers
  const handleNumericInput = (key, value) => {
    const min = getMinValue(key);
    const max = getMaxValue(key);
    const clampedValue = Math.min(Math.max(value || min, min), max);
    
    currentToolValues.value[key] = clampedValue;
    emit('update-tool-values', { ...currentToolValues.value });
  };
  
  const handleColorInput = (event, key) => {
    const color = event.target?.value || event;
    currentToolValues.value[key] = color;
    emit('update-tool-values', { ...currentToolValues.value });
  };
  
  // Tool Value Management
  const getMinValue = (key) => {
    if (props.selectedTool.name === 'crop') {
      if (key.startsWith('start') || key.startsWith('end')) return 0;
    }
    if (key === 'brightness' || key === 'contrast') return -100;
    if (key === 'angle') return -180;
    if (key === 'blurRadius' || key === 'borderSize') return 0;
    if (key.includes('scale')) return 1;
    return 0;
  };
  
  const getMaxValue = (key) => {
    if (props.selectedTool.name === 'crop') {
      if (key.includes('X')) return 10000;
      if (key.includes('Y')) return 10000;
    }
    if (key === 'borderSize') {
      return Math.min(props.imageSize.width || 100, props.imageSize.height || 100) / 2;
    }
    if (key === 'brightness' || key === 'contrast') return 100;
    if (key === 'angle') return 180;
    if (key === 'blurRadius') return 20;
    if (key.includes('scale')) return 1000;
    return 100;
  };
  
  const getStep = (key) => {
    if (key === 'angle') return 1;
    if (key === 'blurRadius') return 0.5;
    if (key.includes('scale')) return 1;
    if (key === 'contrast') return 0.1;
    return 1;
  };
  
  const getUnit = (key) => {
    if (key.includes('scale')) return '%';
    if (key === 'angle') return '°';
    if (key === 'blurRadius' || key === 'borderSize' || key.includes('X') || key.includes('Y')) return 'px';
    return '';
  };
  
  const formatLabel = (key) => {
    return key
      .replace(/([A-Z])/g, ' $1')
      .replace(/^./, (str) => str.toUpperCase());
  };
  
  const selectTool = (tool) => {
    currentToolValues.value = getDefaultValuesForTool(tool.name);
    emit('select-tool', tool);
  };
  
  const resetToolValues = () => {
    currentToolValues.value = getDefaultValuesForTool(props.selectedTool.name);
    emit('update-tool-values', { ...currentToolValues.value });
  };
  
  const addToChain = () => {
    emit('add-to-chain', { ...currentToolValues.value });
  };
  
  const getDefaultValuesForTool = (toolName) => {
    const maxBorderSize = Math.min(props.imageSize.width || 100, props.imageSize.height || 100) / 4;
    
    switch (toolName) {
      case 'crop':
        return {
          startX: 0,
          startY: 0,
          endX: props.imageSize.width || 100,
          endY: props.imageSize.height || 100
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
  
  watch(() => props.selectedTool, (newTool) => {
    if (newTool) {
      currentToolValues.value = { ...newTool.values };
    }
  }, { immediate: true, deep: true });
  
  watch(() => props.imageSize, () => {
    if (props.selectedTool) {
      currentToolValues.value = getDefaultValuesForTool(props.selectedTool.name);
    }
  }, { deep: true });
  </script>