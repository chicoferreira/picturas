<template>
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <!-- Outer circle -->
      <circle cx="12" cy="12" r="10" />
      <!-- Half division for grayscale effect -->
      <path d="M12 2 A10 10 0 0 1 12 22" />
    </svg>
  </template>
  
  <script>
  export default {
    name: 'GrayscaleIcon',
  };
  </script>
  