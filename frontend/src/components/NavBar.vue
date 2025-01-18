<script setup lang="ts">
import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
} from '@/components/ui/navigation-menu'
import { useRouter } from 'vue-router'

const router = useRouter()

// Gerar links dinamicamente com base nas rotas do ficheiro de router
const links = router.options.routes.map((route) => ({
  name: route.name,
  href: route.path,
}))

// Opcional: Filtrar rotas específicas se não quiseres todas no menu
const menuLinks = links.filter(
  (link) => link.name !== 'TestePage' && link.name !== 'Register' && link.name !== 'Login',
)
</script>

<template>
  <div class="navbar-container">
    <NavigationMenu class="navigation-menu">
      <NavigationMenuList>
        <NavigationMenuItem v-for="link in menuLinks" :key="link.name">
          <NavigationMenuLink :href="link.href">{{ link.name }}</NavigationMenuLink>
        </NavigationMenuItem>
      </NavigationMenuList>
    </NavigationMenu>
  </div>
</template>

<style scoped>
.navbar-container {
  display: flex;
  justify-content: flex-start;
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
