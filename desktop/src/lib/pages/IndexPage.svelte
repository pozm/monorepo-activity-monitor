<script lang="ts">
	import { app, event, http, invoke } from "@tauri-apps/api";
	import { Data } from "../store";
	import { fade, fly } from "svelte/transition";
	import type { GithubTagApi } from "../DataTypes";
	import {
		getLatestLauncherTag,
		getLatestVulnusTag,
		getPercent,
		getTagDownload,
		getTagFromRef,
		installVersion,
		launchVulnus,
		removeVersion,
		versionInstalled,
	} from "../SharedFunctions";
	import {formatRelative,formatDistance} from 'date-fns'
	import { Applications, ShowNewModal } from "../StoreData";
	import plusIco from '../../assets/svg/plusico.svg';
import { beforeUpdate, onDestroy, onMount } from "svelte";
import { convertFileSrc } from "@tauri-apps/api/tauri";
	let l : Promise<event.UnlistenFn>
	onMount(() => {
		l = event.listen("backend://activity-update",async (data) => {
			let p = data.payload as {[x:string]:string}


			console.log("update pog",$Applications,p)

			for (let v of Object.values($Applications)) {
				v.open=false;
				v.dateOpen=null;
				let g = p[v.location] ?? null
				if (g) {
					v.open=true;
					console.log(g+" UTC")
					v.dateOpen=new Date(g+"Z");
				}
			}
			Applications.set($Applications)
		})

	})
	onDestroy(()=>{
		l.then(v=>{
			v();
		})
	})

	//#region store.load
	event.once("client://store-loaded", async () => {
		console.log("store loaded../");
		// let lastUpdate = Number(
		// 	new Date(Data.Store.get.data.vulnus.version.last_check as string).getTime() ?? 0
		// );
		// if (lastUpdate < Date.now() - 1e3 * 60 * 15) {
		// 	await http.fetch<GithubTagApi.RootObject[]>(
		// 		"https://api.github.com/repos/beat-game-dev/Vulnus/git/refs/tags"
		// 	).then((rdata) => {
		// 		// console.log("new err",rdata.data)
		// 		// console.log(versions,rdata) //
		// 		VersionsAvailable.set(
		// 			rdata.data.map((v) => getTagFromRef(v.ref))
		// 		);
		// 		Data.Store.get.data.vulnus.version.last_check = new Date(Date.now()).toISOString();
		// 		Data.Store.get.data.vulnus.version.versions = $VersionsAvailable
		// 		// Data.Store.get.add(
		// 		// 	"Vulnus.versions.last_check",
		// 		// 	Date.now().toString()
		// 		// );
		// 		// Data.Store.get.add(
		// 		// 	"Vulnus.versions",
		// 		// 	JSON.stringify($VersionsAvailable)
		// 		// );
		// 	});
		// 	await getLatestVulnusTag().then((tag) => {
		// 		LatestVersionsAvailable.set(tag);
		// 		console.log("latest",tag)
		// 		// Data.Store.get.add("Vulnus.versions.latest", tag);
		// 		Data.Store.get.data.vulnus.version.latest = tag
		// 		if (!$ChosenVersion) {
		// 			ChosenVersion.set(tag)
		// 			Data.Store.get.data.vulnus.version.current = tag
		// 			// Data.Store.get.add("Vulnus.versions.chosen", tag);
		// 		}
		// 	});
		// 	await getLatestLauncherTag().then((tag) => {
		// 		app.getVersion().then((ver) => {
		// 			console.log("app ver", ver);
		// 			if (tag.includes(ver)) {
		// 				console.log("already updated");
		// 			} else {
		// 				event.emit("client://notification", {
		// 					title: `Update Available`,
		// 					data: `Go to my github to get the latest release!`,
		// 				});
		// 			}
		// 		});
		// 		Data.Store.get.data.launcher.latest_version = tag
		// 		// Data.Store.get.add("Launcher.versions.latest", tag);
		// 	});
		// } else {
		// 	console.log(
		// 		`not updating cuz last update too soon (${lastUpdate})`
		// 	);
		// }

		Applications.set(Data.Store.get.data.server.applications);

		Data.Store.get.write()
	});
	//#endregion

</script>

<div class="flex w-full h-full flex-col">
	<!-- {@debug(chosenVersion)}	 -->
	<h1 class="text-gray-200 text-4xl">Activity Monitor</h1>
	<p class="text-gray-400 mb-2">See what applications you have running for a while</p>
	<!-- show game pictures -->
	<div class="relative flex-1 rounded-xl overflow-hidden">

		<div class="flex w-full flex-wrap flex-row justify-around" >
			{#each $Applications as app}
			<div class={`w-80 h-64 bg-zinc-800 rounded-xl p-4 border border-solid ${app.open ? "border-emerald-400" : "border-rose-600"} shadow-lg text-neutral-400  flex flex-col mt-4`} >
					<img src={((app)=>{

						return convertFileSrc(app.icon_location)

					})(app)} class="aspect-square max-w-6 rounded-xl mb-2" width="80" alt="imagea" />
					<h3 class="text-xl" >{app.name}</h3>
					<div class="w-full flex flex-col" >
						<p class="text-ellipsis w-full overflow-y-clip whitespace-nowrap overflow-clip" >Location: <span class="text-ellipsis text-pink-300" >{app.location}</span></p>
						{#if app.open}
							<p>
								Application is currently open. 
							</p>
							<p>Open for : <span class="text-pink-300" >{formatDistance(app.dateOpen,new Date())}</span></p>
						{/if}
					</div>
				</div>
			{/each}


			<div on:click={()=>ShowNewModal.set(true)} class=" cursor-pointer hover:text-zinc-300 hover:border-zinc-600 hover:bg-zinc-700 transition-colors w-52 h-52 items-center justify-center bg-zinc-800 rounded-xl p-4 border border-solid border-zinc-600 shadow-lg text-neutral-400  flex flex-col mt-4" >
				Add new applications
				<svelte:component class="h-24 w-24 text-current" this={plusIco} />
			</div>

		</div>	

		<!-- <div class="absolute h-full w-80 right-0 p-2 ">
		</div> -->
	</div>
</div>

<style scoped>
	.vulnusBg {
		background-size: cover;
	}
</style>
