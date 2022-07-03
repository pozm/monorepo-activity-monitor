<!-- <svelte:options tag="github-link" /> -->

<script lang="ts">

	function OpenGithub_(){
		open("https://github.com/pozm")
	}

</script>
<span class="cursor-pointer text-pink-300" on:click={OpenGithub_} >
	<slot>my github</slot>
</span>