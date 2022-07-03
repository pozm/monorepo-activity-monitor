<script lang="ts">
import { Command } from "@tauri-apps/api/shell";




	function openGithub(user:string) {
		new Command('ope',[`https://github.com/${user}`]).spawn()
		// open(`https://github.com/${user}`)
	}

	let users = [
		{
			name: "Luna",
			desc:"Developer of the Activity Monitor",
			github : "pozm",
			color : "text-pink-300"
		}
	]

</script>


<h1 class="text-gray-200 text-5xl mb-4" >Information</h1>
<p class="text-gray-400" >Hi, thanks for using the Activity Monitor!</p>
<hr class="my-2 border-zinc-700 border rounded-lg" />
<h3 class="text-gray-200 text-3xl mb-4" >Credits</h3>
<div class="block w-full" >
	<p class="text-gray-400" >Activity Monitor wouldn't be possible without the people behind it</p>
	<ul class="text-gray-300 space-y-1 py-2 list-disc list-inside" >
		{#each users as user}
		<li><span class={`${user.color} hover:brightness-75 cursor-pointer`} on:click={openGithub.bind(this,user.github)} >{user.name}</span>: {user.desc}</li>

		{/each}
	</ul>
</div>