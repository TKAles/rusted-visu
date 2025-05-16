<script lang="ts">
	export let title = "VISU | Rusted Edition";
</script>
	
<div class="min-h-screen bg-gray-50">
	<!--- Header -->
	<header class="bg-white shadow">
		<div class="container mx-auto px-4 py-4">
			<h1>{title}</h1>	
		</div>
	</header>
	<!-- Main Window -->
	 <main class="container mx-auto px-4 py-4">
		<slot />
	</main>
	<footer></footer>
</div>
