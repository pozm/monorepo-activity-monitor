<script lang="ts">
import { path, shell } from "@tauri-apps/api";

import { appDir } from "@tauri-apps/api/path";

import { Command } from "@tauri-apps/api/shell";
import { onMount } from "svelte";

import { getDataDir, launcherDir } from "../SharedFunctions";
import { Data } from "../store";
import { ServerAddress, ServerKey, ServerModalType, ShowNewModal, ShowServerModal } from "../StoreData";

	async function openLauncherPath() {
		let dir = `${await launcherDir()}${path.sep}`;
		console.log(dir)
		shell.open(dir)
		// new Command('ope',[dir]).spawn().then(ch=>{
		// 	// the h
		// });
	}
	async function openDataPath() {
		let data_dir = `${await getDataDir()}${path.sep}`
		console.log(data_dir)
		shell.open(data_dir);
		// new Command('ope',[data_dir]).spawn().then(ch=>{
		// 	// the h
		// });
	}



</script>

<div class="space-y-2" >

	<h1 class="text-gray-200 text-5xl mb-4" >Settings</h1>
	<div class="relative w-full">

		<div class="my-4 bg-neutral-900 p-2 border border-solid border-neutral-700 rounded-xl shadow-lg z-30" >
			
			<div class="flex flex-row w-full content-center items-center" >
				<p class="text-gray-300 text-xl mr-auto" >Server Address: <span class="text-pink-300" >{$ServerAddress}</span></p>
				<button on:click={()=>{ShowServerModal.set(true);ServerModalType.set(2)}}  class="ml-auto py-2 shadow-sm px-12 transition-colors hover:bg-cyan-600 text-gray-100 bg-cyan-500 disabled:bg-cyan-600/50 mt-2 rounded-lg">Change</button>
			</div>
			<div class="flex flex-row w-full content-center items-center" >
				<p class="text-gray-300 text-xl mr-auto" >Server ApiKey: <span class="text-pink-300 break-all" >{$ServerKey}</span></p>
				<button on:click={()=>{ShowServerModal.set(true);ServerModalType.set(1)}}  class="ml-auto py-2 shadow-sm px-12 transition-colors hover:bg-cyan-600 text-gray-100 bg-cyan-500 disabled:bg-cyan-600/50 mt-2 rounded-lg">Change</button>
			</div>
		<div class="flex flex-row w-full content-center items-center" >
			<p class="text-gray-300 text-xl mr-auto" >Open launcher folder</p>
			<button on:click={openLauncherPath} class="ml-auto py-2 shadow-sm px-12 transition-colors hover:bg-indigo-600 text-gray-100 bg-indigo-500 disabled:bg-indigo-600/50 mt-2 rounded-lg">Open</button>
		</div>
		<div class="flex flex-row w-full content-center items-center" >
			<p class="text-gray-300 text-xl mr-auto" >Open Data folder</p>
			<button on:click={openDataPath} class="ml-auto py-2 shadow-sm px-12 transition-colors hover:bg-yellow-600 text-gray-100 bg-yellow-500 disabled:bg-yellow-600/50 mt-2 rounded-lg">Open</button>
		</div>
	</div>
</div>
</div>