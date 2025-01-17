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
      path: '/register',
      name: 'Register',
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
    {
      path: '/',
      name: 'home',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('../views/HomeView.vue'),
    },
    {
      path: '/subscriptions',
      name: 'Subscriptions',
      component: () => import('../views/SubscriptionPlans.vue'),
    },
    {
      path: '/projects',
      name: 'Projects',
      component: () => import('../views/ProjectsView.vue'),
    },
    //{
    //  path: '/forms',
    //  name: 'Forms',
    //  component: () => import('../views/FormView.vue'),
    //},
    {
      path: '/settings',
      name: 'Settings',
      component: () => import('../views/SettingsPage.vue'),
    },
  ],
})

export default router
