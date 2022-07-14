
<script lang="ts">
	import Modal from './Components/Modal.svelte';
import { dialog,fs,invoke, path } from "@tauri-apps/api";

import IndexPage from "./lib/pages/IndexPage.svelte";
// import { Data } from "./lib/store";
import { ShowNewModal, Applications, ServerAddress, ServerKey , ShowServerModal, ServerModalType} from './lib/StoreData'
import {event} from '@tauri-apps/api'
import NotificationHandler from './Components/NotificationHandler.svelte';
import { onDestroy, onMount } from 'svelte';
import home_ico from './assets/svg/homeico.svg'
import settings_ico from './assets/svg/settingsico.svg'
import info_ico from './assets/svg/infoico.svg'
import add_ico from './assets/svg/plusico.svg'
import SettingsPage from './lib/pages/SettingsPage.svelte';
// import ModPage from './lib/pages/ModPage.svelte';
import { fade, fly } from 'svelte/transition';
// import { LatestVersionsAvailable,ChosenVersion, VersionsAvailable } from './lib/StoreData';
import { Data } from './lib/store';
import InfoPage from './lib/pages/InfoPage.svelte';
import type { DialogFilter } from '@tauri-apps/api/dialog';
import type { IDataStoreTypes } from './lib/DataTypes';

	let updatePath = "";
	let updateSource = "";
	let PathActive = false;
	let awaitingData : ReturnType<typeof Data.Store.get.reload>
	onMount(async ()=>{

		awaitingData = Data.Store.get.reload();
		awaitingData.then((data:IDataStoreTypes)=>{
			// updatePath = data["Vulnus.path"]
			console.log("got data: ",data)
			
			ServerAddress.set(data.server.address)
			ServerKey.set(data.server.api_key)


			event.emit("client://store-loaded")
		})
	})

	let PathIsInvalida = false;
	let PathIsInvalidb = false;
	let PagesMap = [
		{c:IndexPage,s:home_ico,n:"Home"},
		// {c:ModPage,s:add_ico,n:"Modding"},
		{c:InfoPage,s:info_ico,n:"Info"},
		{c:SettingsPage,s:settings_ico,n:"Settings"},
	]
	let ShowPage = 0;

	let usableget = true;
	let SidebarHovering = false;


	let newappn = "";
	let newappp = "";
	let newappi = "";




	function GetPath(x:string,f?:DialogFilter[],u=false) {
		console.log("GetVulnusPath");
		return dialog.open({
			directory:false,
			filters:f,
			defaultPath:x,
			title:"Select Vulnus path",
			multiple:false
		}).then(v=>{
			if (newappn == "" && u) {
				path.basename(v as string,"exe").then(v=>{
					newappn = v.slice(0,-1);
				})
			}
			return v as string;
		})
	}

	function make_application() {
		console.log("make_application",newappn,newappp,newappi);
		invoke("new_application",{name:newappn,icon:newappi,location:newappp}).then(async v=>{
			await Data.Store.get.reload();
			Applications.set(Data.Store.get.data.server.applications);
		})
		newappi="";
		newappn="";
		newappp="";
	}
	let new_server_value = "";
	function alter_server_settings() {
		let cmd = "q"
		if ($ServerModalType == 1)
		{
			cmd = "set_api_key"
			ServerKey.set(new_server_value)
		}
		else if ($ServerModalType == 2)
		{
			cmd = "set_server_addr"
			ServerAddress.set(new_server_value)
		}
		invoke(cmd,{st:new_server_value})
		new_server_value = "";
	}


	$: {
		invoke<boolean>("dir_exist",{dir: newappi}).then(d=>{
			PathIsInvalida=!d;
			console.log(PathIsInvalida)
		})
		invoke<boolean>("dir_exist",{dir: newappp}).then(d=>{
			PathIsInvalidb=!d;
			console.log(PathIsInvalidb)
		})
		if ($ShowNewModal == true && usableget) {
			usableget = false
		}
	}

	
</script>
<div class="flex min-h-screen" >
	<NotificationHandler/>
	<Modal show={$ShowNewModal} data={ShowNewModal} prematureClose={true} >
		<h1 class="text-gray-200 text-2xl" >New Application</h1>
		<div class="relative">

			<label for="path" class="block text-sm font-medium text-gray-400" >Please enter the path of the executable</label>
			<input on:blur={()=>{
				PathActive=false;
			}} on:focus={(v)=>{
				PathActive=true
			}} bind:value={newappp} id="path" name="path" class={`appearance-none w-full text-neutral-200 placeholder-zinc-400 transition-colors bg-neutral-900 focus:outline-none focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-2 shadow-sm border ${PathIsInvalida ? "border-red-400 ring-red-400" : "border-zinc-600"}`} placeholder="Path to Exe">
			<button on:click={async()=> newappp =  await GetPath(newappp,[{name:"executable",extensions:["exe"]}],true)} class="absolute right-2 bottom-2.5 w-6 h-6" >
				<svg xmlns="http://www.w3.org/2000/svg" class={`h-6 w-6 ${PathActive ? "text-pink-200" : "text-zinc-600"}`} fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
					<path stroke-linecap="round" stroke-linejoin="round" d="M8 7v8a2 2 0 002 2h6M8 7V5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V15a2 2 0 01-2 2h-2M8 7H6a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2v-2" />
				</svg>
			</button>
		</div>
		<div class="relative" >

			<label for="name" class="block text-sm font-medium text-gray-400" >Shorted or custom name for the application</label>
			<input on:blur={()=>{
				PathActive=false;
			}} on:focus={(v)=>{
				PathActive=true
			}} bind:value={newappn} id="name" name="name" class={`appearance-none w-full text-neutral-200 placeholder-zinc-400 transition-colors bg-neutral-900 focus:outline-none focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-2 shadow-sm border ${null ? "border-red-400 ring-red-400" : "border-zinc-600"}`} placeholder="Simplified name">
		</div>

		<div class="relative">

			<label for="pathi" class="block text-sm font-medium text-gray-400" >Please enter the path of the image</label>
			<input on:blur={()=>{
				PathActive=false;
			}} on:focus={(v)=>{
				PathActive=true
			}} bind:value={newappi} id="pathi" name="pathi" class={`appearance-none w-full text-neutral-200 placeholder-zinc-400 transition-colors bg-neutral-900 focus:outline-none focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-2 shadow-sm border ${PathIsInvalidb ? "border-red-400 ring-red-400" : "border-zinc-600"}`} placeholder="Path to Image">
			<button on:click={async()=> newappi =  await GetPath(newappi,[{name:"image",extensions:["png","jpg","webp"]}],true)} class="absolute right-2 bottom-2.5 w-6 h-6" >
				<svg xmlns="http://www.w3.org/2000/svg" class={`h-6 w-6 ${PathActive ? "text-pink-200" : "text-zinc-600"}`} fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
					<path stroke-linecap="round" stroke-linejoin="round" d="M8 7v8a2 2 0 002 2h6M8 7V5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V15a2 2 0 01-2 2h-2M8 7H6a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2v-2" />
				</svg>
			</button>
		</div>


		<div class="flex justify-end" >
			{#if PathIsInvalida || PathIsInvalidb}
				<p class="mr-auto mt-2 text-sm text-red-400 select-none " >The path(s) you have provided is is invalid </p>
				
			{/if}
			<div class="px-2" >
				<button class="py-2 shadow-sm px-8 transition-colors hover:bg-red-600 text-gray-100 bg-red-500 disabled:bg-red-600/50 mt-2 rounded-lg" on:click="{()=>{ShowNewModal.set(false)}}">Close</button>
				<button disabled={PathIsInvalida || PathIsInvalidb} class="py-2 shadow-sm px-8 transition-colors hover:bg-green-600 text-gray-100 bg-emerald-500 disabled:bg-emerald-600/50 mt-2 rounded-lg" on:click="{make_application}">Save</button>
			</div>
		</div>
	</Modal>

	<Modal show={$ShowServerModal} data={ShowServerModal} prematureClose={true} >
		<h1 class="text-gray-200 text-2xl" >Change {$ServerModalType == 1 ? "Api Key" : $ServerModalType == 2 ? "Server Address" : "what"}</h1>

		<div class="relative" >

			<label for="name" class="block text-sm font-medium text-gray-400" >The new value of the {$ServerModalType == 1 ? "Api Key" : $ServerModalType == 2 ? "Server Address" : "what"}</label>
			<input on:blur={()=>{
				PathActive=false;
			}} on:focus={(v)=>{
				PathActive=true
			}} bind:value={new_server_value} id="name" name="name" class={`appearance-none w-full text-neutral-200 placeholder-zinc-400 transition-colors bg-neutral-900 focus:outline-none focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-2 shadow-sm border ${null ? "border-red-400 ring-red-400" : "border-zinc-600"}`} placeholder={`new ${$ServerModalType == 1 ? "Api Key" : $ServerModalType == 2 ? "Server Address" : "what"}`}>
		</div>

		<div class="flex justify-end" >
			<!-- {#if PathIsInvalida || PathIsInvalidb}
				<p class="mr-auto mt-2 text-sm text-red-400 select-none " >The path(s) you have provided is is invalid </p>
				
			{/if} -->
			<div class="px-2" >
				<button class="py-2 shadow-sm px-8 transition-colors hover:bg-red-600 text-gray-100 bg-red-500 disabled:bg-red-600/50 mt-2 rounded-lg" on:click="{()=>{ShowServerModal.set(false)}}">Close</button>
				<button class="py-2 shadow-sm px-8 transition-colors hover:bg-green-600 text-gray-100 bg-emerald-500 disabled:bg-emerald-600/50 mt-2 rounded-lg" on:click="{alter_server_settings}">Save</button>
			</div>
		</div>
	</Modal>
	
	<div class="min-h-screen relative select-none w-14 bg-zinc-800 hover:w-32 transition-all duration-200 px-1" on:mouseleave={()=>SidebarHovering=false} on:mouseenter={()=>SidebarHovering=true} >
		<div class="fixed max-w-full h-full flex flex-col py-2 items-center hover:items-start transition-all duration-200 space-y-2" >

			<!-- {@debug PagesMap} -->
			{#each PagesMap as page,i}
			{@const selected = ShowPage == i}
			<div class={`py-2 cursor-pointer w-full flex flex-row ${!SidebarHovering ? "justify-center" : "justify-start"} ${selected? "bg-zinc-900/50" : ""} hover:bg-zinc-900/90  rounded-lg px-2 lastBottom`} on:click={()=>ShowPage=i} >
				<svelte:component class="h-7 w-7 text-gray-300" this={page.s} />
				{#if SidebarHovering}
				<p in:fly={{x:-20}} out:fly={{x:-20,duration:200}} class="text-neutral-400 flex items-center w-full justify-end" >{page.n}</p>
				
				{/if}
			</div>
			{/each}
		</div>
	</div>
	
	<div class="p-4 w-full" >
		<!-- <h1 class="text-gray-200 text-xl" >Vulnus Mod Assistant</h1> -->
		<svelte:component this={PagesMap[ShowPage].c} />
	</div>
</div>

<style scoped >
	.lastBottom:last-child {
		margin-top:auto !important;
	}
</style>