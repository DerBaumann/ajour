<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
  <header>
    <nav>
      <ul>
        <li>
          <RouterLink to="/">Home</RouterLink>
        </li>
        <li>
          <RouterLink to="/tasks">Heutige Aufgaben</RouterLink>
        </li>
      </ul>
    </nav>
  </header>

  <RouterView />
</template>
