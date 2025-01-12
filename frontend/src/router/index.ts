import { createRouter, createWebHistory } from 'vue-router'
/*import { useSessionStorage} from "@/stores/sessionStorage"; */

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/teste',
      name: 'TestePage',
      component: () => import('../views/TesteHomeView.vue'),
    },
    {
      path: '/createacc',
      name: 'CreateAccount',
      component: () => import('../views/CreateAccount.vue'),
    },
    {
      path: '/login',
      name: 'Login',
      component: () => import('../views/LoginView.vue'),
    },
    {
      path: '/about',
      name: 'about',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('../views/AboutView.vue'),
    },
  ],
})

export default router
