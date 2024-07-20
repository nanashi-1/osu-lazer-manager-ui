<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { Button, Spinner } from "flowbite-svelte";
  import { DownloadSolid, PlaySolid } from "flowbite-svelte-icons";

  export let selectedVersion: string;
  export let currentVersionInstalled: boolean;

  // Check if current version is installed
  $: invoke("is_installed", { version: selectedVersion })
    .then((value) => {
      /** @ts-expect-error */
      currentVersionInstalled = value;
    })
    .catch((error) => {
      console.error(error);
    });

  export let downloadInProgress: boolean;
  export let install: () => void;
</script>

{#if currentVersionInstalled && !downloadInProgress}
  <Button
    class="p-2 pe-3 font-bold"
    on:click={() =>
      invoke("launch", { version: selectedVersion }).catch((error) =>
        console.error(error),
      )}
  >
    <PlaySolid class="h-5 w-5" />Launch
  </Button>
{:else if !currentVersionInstalled && !downloadInProgress}
  <Button class="p-2 pe-3 font-bold" on:click={install}>
    <DownloadSolid class="h-5 w-5" />Install
  </Button>
{:else}
  <Button class="p-2 pe-3 font-bold" disabled outline>
    <Spinner class="me-1" size="4" />Installing
  </Button>
{/if}
