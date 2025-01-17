<template>
    <div class="min-h-screen bg-[#030712] flex items-center justify-center px-4 py-12">
      <div class="w-full max-w-[1200px] 2xl:max-w-[1600px] mx-auto">
        <!-- Logo and Header -->
        <div class="text-center space-y-4 mb-12">
          <h1 class="text-5xl font-bold bg-gradient-to-r from-[#6D28D9] to-white bg-clip-text text-transparent tracking-tight">PICTURAS</h1>
          <h2 class="text-4xl font-bold text-white">Plans & Pricing</h2>
          <p class="text-[#969696] text-lg">
            Choose the perfect plan for your creative journey
          </p>
        </div>
  
        <!-- Pricing Cards -->
        <div class="grid md:grid-cols-3 gap-6 lg:gap-8 2xl:gap-12 max-w-5xl 2xl:max-w-7xl mx-auto">
          <!-- Anonymous Plan -->
          <div 
            class="bg-[#0f1629] rounded-lg p-6 2xl:p-8 flex flex-col min-h-[480px] 2xl:min-h-[600px] relative transition-all duration-300 hover:scale-105 hover:border hover:border-[#6D28D9] hover:shadow-2xl hover:shadow-[#6D28D9]/20"
          >
            <div class="space-y-2">
              <h3 class="text-2xl 2xl:text-3xl font-bold text-white">Anonymous</h3>
              <p class="text-[#969696] 2xl:text-lg">Get started without registration</p>
            </div>
            
            <div class="mt-4">
              <p class="text-3xl 2xl:text-4xl font-bold text-white">Free</p>
            </div>
  
            <div class="space-y-4 2xl:space-y-6 mt-8 flex-grow">
              <div class="flex items-center text-white">
                <HardDriveIcon class="h-5 w-5 2xl:h-6 2xl:w-6 mr-3 text-[#6D28D9]" />
                <span>1 GB Storage</span>
              </div>
              <div class="flex items-center text-white">
                <FoldersIcon class="h-5 w-5 2xl:h-6 2xl:w-6 mr-3 text-[#6D28D9]" />
                <span>3 Projects</span>
              </div>
              <div class="flex items-center text-white">
                <PencilIcon class="h-5 w-5 2xl:h-6 2xl:w-6 mr-3 text-[#6D28D9]" />
                <span>Basic Editing Tools</span>
              </div>
            </div>
  
            <Button class="w-full bg-[#6D28D9] hover:bg-[#5b21b6] mt-6 2xl:py-6 2xl:text-lg">
              Start Creating
            </Button>
          </div>
  
          <!-- Registered Plan -->
          <div 
            class="bg-[#0f1629] rounded-lg p-6 2xl:p-8 flex flex-col min-h-[480px] 2xl:min-h-[600px] relative border border-[#6D28D9] transition-all duration-300 hover:scale-105 hover:shadow-2xl hover:shadow-[#6D28D9]/20"
          >
            <div class="absolute -top-3 left-1/2 -translate-x-1/2">
              <Badge class="bg-[#6D28D9] text-white">Most Popular</Badge>
            </div>
  
            <div class="space-y-2">
              <h3 class="text-2xl 2xl:text-3xl font-bold text-white">Registered</h3>
              <p class="text-[#969696] 2xl:text-lg">Perfect for regular creators</p>
            </div>
            
            <div class="mt-4">
              <p class="text-3xl 2xl:text-4xl font-bold text-white">Free</p>
            </div>
  
            <div class="space-y-4 2xl:space-y-6 mt-8 flex-grow">
              <div class="flex items-center text-white">
                <HardDriveIcon class="h-5 w-5 2xl:h-6 2xl:w-6 mr-3 text-[#6D28D9]" />
                <span>5 GB Storage</span>
              </div>
              <div class="flex items-center text-white">
                <FoldersIcon class="h-5 w-5 2xl:h-6 2xl:w-6 mr-3 text-[#6D28D9]" />
                <span>10 Projects</span>
              </div>
              <div class="flex items-center text-white">
                <WrenchIcon class="h-5 w-5 2xl:h-6 2xl:w-6 mr-3 text-[#6D28D9]" />
                <span>Basic & Advanced Tools</span>
              </div>
              <div class="flex items-center text-white">
                <UserIcon class="h-5 w-5 2xl:h-6 2xl:w-6 mr-3 text-[#6D28D9]" />
                <span>Personal Profile</span>
              </div>
            </div>
  
            <Button class="w-full bg-[#6D28D9] hover:bg-[#5b21b6] mt-6 2xl:py-6 2xl:text-lg">
              Create Account
            </Button>
          </div>
  
          <!-- Premium Plan -->
          <div 
            class="bg-[#0f1629] rounded-lg p-6 2xl:p-8 flex flex-col min-h-[480px] 2xl:min-h-[600px] relative transition-all duration-300 hover:scale-105 hover:border hover:border-[#6D28D9] hover:shadow-2xl hover:shadow-[#6D28D9]/20"
          >
            <div class="space-y-2">
              <h3 class="text-2xl 2xl:text-3xl font-bold text-white">Premium</h3>
              <p class="text-[#969696] 2xl:text-lg">For professional creators</p>
            </div>
            
            <div class="mt-4">
              <p class="text-3xl 2xl:text-4xl font-bold text-white">
                $9.99<span class="text-lg text-[#969696]">/month</span>
              </p>
            </div>
  
            <div class="space-y-4 2xl:space-y-6 mt-8 flex-grow">
              <div class="flex items-center text-white">
                <InfinityIcon class="h-5 w-5 2xl:h-6 2xl:w-6 mr-3 text-[#6D28D9]" />
                <span>Unlimited Storage</span>
              </div>
              <div class="flex items-center text-white">
                <InfinityIcon class="h-5 w-5 2xl:h-6 2xl:w-6 mr-3 text-[#6D28D9]" />
                <span>Unlimited Projects</span>
              </div>
              <div class="flex items-center text-white">
                <SparklesIcon class="h-5 w-5 2xl:h-6 2xl:w-6 mr-3 text-[#6D28D9]" />
                <span>All Tools & Features</span>
              </div>
              <div class="flex items-center text-white">
                <CrownIcon class="h-5 w-5 2xl:h-6 2xl:w-6 mr-3 text-[#6D28D9]" />
                <span>Priority Support</span>
              </div>
              <div class="flex items-center text-white">
                <StarIcon class="h-5 w-5 2xl:h-6 2xl:w-6 mr-3 text-[#6D28D9]" />
                <span>Early Access to New Features</span>
              </div>
            </div>
  
            <Button class="w-full bg-[#6D28D9] hover:bg-[#5b21b6] mt-6 2xl:py-6 2xl:text-lg">
              Upgrade to Premium
            </Button>
          </div>
        </div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { Button } from '@/components/ui/button'
  import { Badge } from '@/components/ui/badge'
  import {
    HardDriveIcon,
    FoldersIcon,
    PencilIcon,
    WrenchIcon,
    UserIcon,
    InfinityIcon,
    SparklesIcon,
    CrownIcon,
    StarIcon
  } from 'lucide-vue-next'
  </script>
  
  