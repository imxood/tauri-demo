<script lang="ts">
  import { appWindow, getCurrent } from "@tauri-apps/api/window";
  import { exit, relaunch } from "@tauri-apps/api/process";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import Greet from "./lib/Greet.svelte";

  import data from "./2019.js";
  import BarChart from "./BarChart.svelte";
  import D3Demo from "./D3Demo.svelte";

  const minConsoleHeight = 50;

  let isWindowMaximized = false;
  let isDark = true;
  let console_ele: HTMLElement;
  let enable_terminal = false;

  onMount(async () => {
    // 加载主题
    isDark = localStorage && localStorage.getItem("theme") == "dark";
    applyTheme(isDark);

    const window = getCurrent();
    isWindowMaximized = await window.isMaximized();
    listen("tauri://resize", async () => {
      isWindowMaximized = await window.isMaximized();
    });
  });

  let cStartY: number;
  let consoleH: number;

  function startResizingConsole(e: MouseEvent) {
    cStartY = e.clientY;
    console.log("cStartY", cStartY);

    const styles = window.getComputedStyle(console_ele);
    consoleH = parseInt(styles.height, 10);

    const moveHandler = (e: MouseEvent) => {
      const dy = e.clientY - cStartY;
      const newH = consoleH - dy;
      console.log("dy", dy);
      console_ele.style.height = `${
        newH < minConsoleHeight ? minConsoleHeight : newH
      }px`;
    };
    const upHandler = () => {
      document.removeEventListener("mouseup", upHandler);
      document.removeEventListener("mousemove", moveHandler);
    };
    document.addEventListener("mouseup", upHandler);
    document.addEventListener("mousemove", moveHandler);
  }

  function applyTheme(isDark) {
    const html = document.querySelector("html");
    isDark ? html.classList.add("dark") : html.classList.remove("dark");
    localStorage && localStorage.setItem("theme", isDark ? "dark" : "");
  }

  function toggleDark() {
    isDark = !isDark;
    applyTheme(isDark);
  }

  async function minimize() {
    await getCurrent().minimize();
  }

  async function closeApp() {
    await exit();
  }

  async function relaunchApp() {
    await relaunch();
  }

  async function toggleMaximize() {
    const window = getCurrent();
    (await window.isMaximized()) ? window.unmaximize() : window.maximize();
  }

  async function close() {
    await getCurrent().close();
  }
</script>

<div
  class="w-100% absolute top-0 left-0 select-none h-8 pl-2 flex justify-between items-center
      text-primaryText dark:text-darkPrimaryText bg-titlebar dark:bg-darkTitlebar"
  data-tauri-drag-region
>
  <span>Tauri API Validation</span>
  <span
    class="h-100% children:h-100% children:w-12 children:inline-flex children:items-center children:justify-center"
  >
    <span
      title={isDark ? "Switch to Light mode" : "Switch to Dark mode"}
      class="hover:bg-hoverOverlay active:bg-hoverOverlayDarker dark:hover:bg-darkHoverOverlay dark:active:bg-darkHoverOverlayDarker"
      on:click={toggleDark}
    >
      {#if isDark}
        <div class="i-ph-sun" />
      {:else}
        <div class="i-ph-moon" />
      {/if}
    </span>
    <span
      title="Minimize"
      class="hover:bg-hoverOverlay active:bg-hoverOverlayDarker dark:hover:bg-darkHoverOverlay dark:active:bg-darkHoverOverlayDarker"
      on:click={minimize}
    >
      <div class="i-codicon-chrome-minimize" />
    </span>
    <span
      title={isWindowMaximized ? "Restore" : "Maximize"}
      class="hover:bg-hoverOverlay active:bg-hoverOverlayDarker dark:hover:bg-darkHoverOverlay dark:active:bg-darkHoverOverlayDarker"
      on:click={toggleMaximize}
    >
      {#if isWindowMaximized}
        <div class="i-codicon-chrome-restore" />
      {:else}
        <div class="i-codicon-chrome-maximize" />
      {/if}
    </span>
    <span
      title="Close"
      class="hover:bg-red-700 dark:hover:bg-red-700 hover:text-darkPrimaryText active:bg-red-700/90 dark:active:bg-red-700/90 active:text-darkPrimaryText  "
      on:click={close}
    >
      <div class="i-codicon-chrome-close" />
    </span>
  </span>
</div>

<main
  class="w-100% flex-1 overflow-hidden p-t-8 grid grid-rows-[1fr_auto]
  text-primaryText dark:text-darkPrimaryText bg-primary dark:bg-darkPrimary"
>
  <div class="overflow-hidden flex flex-col">
    <div>
      <h1>Welcome to Tauri!</h1>
    </div>

    <div class="select-none">
      <button class="btn" on:click={closeApp}>Close application</button>
      <button class="btn" on:click={relaunchApp}>Relaunch application</button>
      <label>
        <input type="checkbox" bind:checked={enable_terminal} /> 启用控制台
      </label>
    </div>

    <div class="flex-1">
      <D3Demo />
    </div>
  </div>

  {#if enable_terminal}
    <div
      bind:this={console_ele}
      id="console"
      class="select-none h-15rem grid grid-rows-[2px_2rem_1fr] gap-1 overflow-hidden"
    >
      <div
        on:mousedown={startResizingConsole}
        class="bg-black/20 h-1 cursor-ns-resize"
      />

      <div class="flex justify-between items-center px-2">
        <p class="font-semibold">Console</p>
        <div
          class="cursor-pointer h-85% rd-1 p-1 flex justify-center items-center
            hover:bg-hoverOverlay dark:hover:bg-darkHoverOverlay
            active:bg-hoverOverlay/25 dark:active:bg-darkHoverOverlay/25"
        >
          <div class="i-codicon-clear-all" />
        </div>
      </div>

      <div class="px-2 overflow-y-auto all:font-mono code-block all:text-xs" />
    </div>
  {/if}
</main>
