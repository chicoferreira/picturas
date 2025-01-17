<script setup lang="ts">
import { Button } from '@/components/ui/button'
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from '@/components/ui/accordion'
const defaultValues = 'item-1'
const accordionItems = [
  { value: 'item-1', title: 'What is this build', content: 'Content 1' },
  { value: 'item-2', title: 'Is it acessivel', content: 'Content 2' },
]
</script>

<template>
  <div class="flex w-full justify-center">
    <img alt="Vue logo" class="logo" src="@/assets/logo.svg" width="125" height="125" />
    <img alt="Vue logo" class="logo" src="@/assets/logo.svg" width="125" height="125" />
    <Button>Click me</Button>
    <Button variant="link">
    Link
  </Button>
    <p class="text-red-500">{{ 'ola' }}</p>
  </div>
  <Accordion type="single" class="w-full" :default-value="defaultValues">
    <AccordionItem v-for="item in accordionItems" :key="item.value" :value="item.value">
      <AccordionTrigger>{{ item.title }}</AccordionTrigger>
      <AccordionContent>{{ item.content }}</AccordionContent>
    </AccordionItem>
  </Accordion>
</template>

<style scoped>
header {
  line-height: 1.5;
  max-height: 100vh;
}

/*
<script setup lang="ts">
import { Button } from '@/components/ui/button'
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from 'radix-vue/namespaced';
defineProps<{
  msg: string
}>()
const defaultValues = 'item-1'
const accordionItems = [
  {value: 'item-1', title: 'What is this build', content: 'Content 1'},
]
</script>

<template>
  <div class="flex justify-center items-center h-screen">
    <Button>Click me</Button>
  </div>
  <Accordion type="single" class = "w-full" :default-value="defaultValues">
    <AccordionItem v-for="item in accordionItems" :key="item.value" :value="item.value">
    <AccordionTrigger>{{ item.title }}</AccordionTrigger>
    <AccordionContent>{{ item.content }}</AccordionContent>
    </AccordionItem>
  </Accordion>
</template>
*/

.logo {
  display: block;
  margin: 0 auto 2rem;
}

nav {
  width: 100%;
  font-size: 12px;
  text-align: center;
  margin-top: 2rem;
}

nav a.router-link-exact-active {
  color: var(--color-text);
}

nav a.router-link-exact-active:hover {
  background-color: transparent;
}

nav a {
  display: inline-block;
  padding: 0 1rem;
  border-left: 1px solid var(--color-border);
}

nav a:first-of-type {
  border: 0;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }

  nav {
    text-align: left;
    margin-left: -1rem;
    font-size: 1rem;

    padding: 1rem 0;
    margin-top: 1rem;
  }
}
</style>
