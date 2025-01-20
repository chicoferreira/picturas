<script setup lang="ts">
import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
} from '@/components/ui/navigation-menu'
import { useRouter } from 'vue-router'
import { useUserStore } from '@/stores/user'


const router = useRouter()

const links = router.options.routes.map((route) => ({
  name: route.name,
  href: route.path,
}))

// Filtrar links
const menuLinks = links.filter((link) => {
  if (!useUserStore().loggedIn()) {
    // Excluir rotas para utilizadores não autenticados
    return (
      link.name !== 'home' &&
      link.name !== 'Settings' &&
      link.name !== 'ResetPassword' &&
      link.name !== 'Projects' &&
      link.name !== 'Subscriptions'
    )
  } else {
    // Excluir rotas utilizadores autenticados
    return (
      link.name !== 'Register' &&
      link.name !== 'Login' &&
      link.name !== 'home' &&
      link.name !== 'ResetPassword'
    )
  }})

const viewLogout = () => {
  useUserStore().logout()
  router.push('/')
}

</script>

<template>
  <div class="navbar-container flex justify-between items-center">
    <a class="mr-auto ml-2" href="/">
        <h1
            class="text-2xl font-bold bg-gradient-to-r from-[#6D28D9] to-white bg-clip-text text-transparent tracking-tight"
        >
            PICTURAS
        </h1>
      </a>
    <NavigationMenu class="navigation-menu flex-1 items-center">
        <NavigationMenuList class="flex items-center">
            <NavigationMenuItem v-for="link in menuLinks" :key="link.name">
                <NavigationMenuLink :href="link.href">{{ link.name }}</NavigationMenuLink>
            </NavigationMenuItem>
            <NavigationMenuItem v-if="useUserStore().loggedIn()">
                <NavigationMenuLink @click="viewLogout()">Logout</NavigationMenuLink>
            </NavigationMenuItem>
        </NavigationMenuList>
    </NavigationMenu>
</div>

</template>

<style scoped>
.navbar-container {
  display: flex;
  justify-content: center;
  background-color: #030712;
  padding: 5px;
}

.navigation-menu {
  display: flex;
  justify-content: center;
}

.navigation-menu a {
  color: #fff;
  text-decoration: none;
  padding: 10px 15px;
  margin: 0 10px;
  display: block;
  border-radius: 5px;
  transition: background-color 0.3s ease;
  text-transform: capitalize;
  font-size: 0.875rem;
}

.navigation-menu a:hover {
  background-color: rgba(133, 76, 223, 0.75);
  transform: scale(1.08);
}
</style>