<script lang="ts">
import { event } from "@tauri-apps/api";
import type { Event } from "@tauri-apps/api/event";

import { onDestroy, onMount } from "svelte";
import type { IClientNotification } from "../lib/DataTypes";
import { fly, fade } from 'svelte/transition';
import { quintOut } from 'svelte/easing';
import { get } from "svelte/store";
import { AppNotifications } from "../lib/StoreData";
import xcircle from '../assets/svg/xcirico.svg'

	let unregisterListener : Promise<event.UnlistenFn>;
	onMount(()=>{
		unregisterListener = event.listen("client://notification",(notifi: Event<string>)=>{
			let parsedData:IClientNotification = JSON.parse(notifi.payload)
			console.log("got notification:",parsedData)
			let at = $AppNotifications.size
			$AppNotifications.set(at,parsedData)
			AppNotifications.set($AppNotifications)
			// console.log(`add ${at}`,notifications)
			setTimeout(()=>{
				$AppNotifications.delete(at)
				AppNotifications.set($AppNotifications)
				// if ($AppNotifications.size == 0) {
				// 	nc=0;
				// } 
				// console.log(`remove ${at}`,notifications)
			},4.3e3)
		})
	})
	onDestroy(()=>{
		unregisterListener.then(fn=>{
			fn(); 
		},e=>{
			console.log("unable to unlisten to notifications.")
		})
	})
	// let notifiArr: IClientNotification[]=[]
	// $: revNoti = [...$AppNotifications].reverse();
	let uses = ["col-span-8","col-span-7"] // so tailwind compiles these styles.
</script>
<!-- {@debug notifications} -->
<div class="absolute h-full w-72 py-2 right-2 z-60" >
	<div class="w-full h-full flex flex-col z-0" >
		{#each [...$AppNotifications] as [idx,notification]}
		{@const err = notification?.err ?? false}
			<!-- {@debug notification} -->
			{#if notification?.data}
				<div in:fade out:fade class="shadow-xl bg-zinc-800 border border-solid border-neutral-600 rounded-xl p-2 mt-2 z-50 w-full overflow-hidden" >
					<div class="grid grid-cols-8 w-full space-x-2 mb-2" >
						{#if err}
							<div class="justify-center flex align-middle items-center" >
								<svelte:component class="h-7 w-7 text-red-400" this={xcircle} />
							</div>
						{/if}
						<div class={`col-span-${err ? "7" : "8"}`} >
							<h2 class="text-gray-300 text-xl font-bold" >{notification.title}</h2>
							<span class="text-gray-400 text-sm w-full break-words" >
								{#if notification.html}
									{@html notification.data}
								{:else}
									{notification.data}
								{/if}
							</span>
						</div>
					</div>
					{#if err}
						<div class="relative" >

							<div class="absolute -left-2 -bottom-2 w-72 h-2 bg-red-400 " ></div>
						</div>
					{/if}
				</div>
			{/if}
		{/each}
	</div>
</div>
<slot/>