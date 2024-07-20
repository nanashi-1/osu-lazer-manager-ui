<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import { CloseCircleOutline } from "flowbite-svelte-icons";
</script>

<button
  class="fixed right-0 top-0 m-1"
  on:click={async () => await appWindow.close()}
>
  <CloseCircleOutline
    class=" h-8 w-8 text-primary-700 hover:text-primary-800"
  />
</button>
