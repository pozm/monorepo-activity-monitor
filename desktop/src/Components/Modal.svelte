<script lang="ts">
import { event } from '@tauri-apps/api';
import type { Event } from '@tauri-apps/api/event';
import type { Writable } from 'svelte/store';

import { fade, scale } from 'svelte/transition';

	export let show = false;
	export let prematureClose = false;
	export let data : Writable<boolean>;

	function onBgClick(){
		if (!prematureClose) return;
		data.set(false)
	}

</script>

{#if show}
	<div in:fade out:fade class="z-20 absolute w-screen h-screen bg-black/60" >
		<div class="w-screen h-screen flex z-10" on:click|self={onBgClick} >
			<div in:scale out:scale class="m-auto center bg-zinc-800 ring-1 ring-zinc-600 rounded-xl w-70 z-30 p-4" >
				<slot>
					<h1 class="text-gray-300 text-lg" >There's no content provided, lol</h1>
				</slot>
			</div>
			
		</div>
	</div>
{/if}