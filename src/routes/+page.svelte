<script lang="ts">
  import { Select, Button, Progressbar, Spinner } from "flowbite-svelte";
  import {
    PlaySolid,
    CloseCircleOutline,
    DownloadSolid,
  } from "flowbite-svelte-icons";
  import { invoke } from "@tauri-apps/api/tauri";
  import { appWindow } from "@tauri-apps/api/window";
  import { listen } from "@tauri-apps/api/event";

  let selectedVersion = "";
  let versions: string[] = [];
  $: versionSelection = versions.map((version) => ({
    name: version,
    value: version,
  }));

  // Get default version
  invoke("get_default_version")
    .then((value) => {
      /** @ts-expect-error */
      selectedVersion = value;
    })
    .catch((error) => {
      console.error(error);
    });

  // Get versions
  invoke("get_versions")
    .then((values) => {
      /** @ts-expect-error */
      versions = values;
    })
    .catch((error) => {
      console.error(error);
    });

  let updateInstallable = false;
  let currentVersionInstalled = true;

  // Check if current version is installed
  $: invoke("is_installed", { version: selectedVersion })
    .then((value) => {
      /** @ts-expect-error */
      currentVersionInstalled = value;
    })
    .catch((error) => {
      console.error(error);
    });

  $: invoke("is_latest_installed")
    .then((value) => {
      updateInstallable =
        !value && selectedVersion !== versions[versions.length - 1];
    })
    .catch((error) => {
      console.error(error);
    });

  let downloadSize = 0;
  let downloadProgress = 0;
  $: progress = Math.round((downloadProgress / downloadSize) * 100);

  $: downloadInProgress = downloadSize > 0 && downloadProgress < downloadSize;
  $: if (downloadSize > 0 && downloadProgress >= downloadSize) {
    downloadSize = 0;
    currentVersionInstalled = true;
  }

  function install() {
    invoke("install", { version: selectedVersion }).catch((error) => {
      console.error(error);
    });
  }

  // Listen for download events
  listen("size", (event) => {
    /** @ts-expect-error */
    downloadSize = event.payload;
  });

  listen("progress", (event) => {
    /** @ts-expect-error */
    downloadProgress = event.payload;
  });
</script>

<div
  class="h-screen w-screen bg-[url('$lib/bg.jpg')] bg-cover bg-center p-1 text-primary-50 text-xs font-bold"
>
  Image used as the background is owned by ppy.<br />
</div>

<div class="fixed bottom-0 h-1/6 w-screen bg-gradient-to-t from-black"></div>

<button
  class="fixed right-0 top-0 m-1"
  on:click={async () => await appWindow.close()}
>
  <CloseCircleOutline
    class=" h-8 w-8 text-primary-700 hover:text-primary-800"
  />
</button>

<div class="fixed bottom-0 flex w-screen flex-col gap-1 p-1">
  {#if downloadInProgress}
    <Progressbar {progress} />
  {/if}

  <div class="flex flex-row gap-1">
    <Select
      placeholder="Select a version"
      items={versionSelection}
      bind:value={selectedVersion}
      disabled={downloadInProgress}
    />

    {#if updateInstallable}
      <Button
        outline
        class="p-2 pe-3 font-bold"
        on:click={() => {
          selectedVersion = versions[versions.length - 1];
          install();
        }}><DownloadSolid class="h-5 w-5" />Update</Button
      >
    {/if}

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
  </div>
</div>
