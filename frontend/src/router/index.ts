import { createRouter, createWebHistory } from 'vue-router'
import { useUserStore } from '@/stores/user'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
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
      meta: { requiresAuth: true },
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
      meta: { requiresAuth: true },
    },
    {
      path: '/reset-password',
      name: 'ResetPassword',
      component: () => import('../views/ChangePass.vue'),
    },
    {
      path: '/login',
      name: 'Login',
      component: () => import('../views/LoginView.vue'),
    },
    {
      path: '/register',
      name: 'Register',
      component: () => import('../views/CreateAccount.vue'),
    },
  ],
})

router.beforeEach((to, from, next) => {
  const userStore = useUserStore();

  // Block logged-in users from accessing the login and register pages
  if ((to.name === 'Login' || to.name === 'Register') && userStore.loggedIn()) {
    next({ name: 'home' }); // Redirect logged-in users to the Home page
  }
  // Check if the route requires authentication
  else if (to.meta.requiresAuth && !userStore.loggedIn()) {
    next({ name: 'Login' }); // Redirect non-logged-in users to the Login page
  } else {
    // Otherwise, allow access
    next();
  }
});

export default router
